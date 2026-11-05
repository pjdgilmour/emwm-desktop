# emwm-wallpaper-tool

Aplicativo gráfico em C++17 com GTKmm 3 para configurar papéis de parede dos quatro workspaces do EMWM. Permite escolher imagens, posicionamento e cor de fundo por monitor e por workspace.

Ao clicar em **Aplicar**, a ferramenta salva as escolhas em `~/.config/emwm/backdrops.resources`, inclui esse arquivo em `~/.Xresources` e aplica a imagem imediatamente com `tellmwm`. Também atualiza o banco de recursos do servidor X com `xrdb -merge`, para preservar a escolha ao reiniciar apenas o WM.

O EMWM restaura as imagens diretamente de seus recursos nativos no início da sessão. Com a área ativa do EMWM Desktop, o controlador recompõe os fundos no login e após eventos de mudança dos monitores, usando as escolhas salvas por saída. Alterar apenas um workspace preserva os demais.

## Dependências e compilação

```bash
sudo apt install build-essential cmake pkg-config libgtkmm-3.0-dev libx11-dev libxrandr-dev x11-xserver-utils
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j2
ctest --test-dir build --output-on-failure
sudo cmake --install build
```

Dependências de execução: GTKmm 3, libX11, libXrandr, `xrdb`, `tellmwm` do EMWM (aplicação imediata). A falta de `xrdb` ou uma falha no comando é informada; a configuração salva continua disponível para o próximo login.

Para instalar somente para seu usuário (sem senha de administrador):

```bash
cmake --install build --prefix "$HOME/.local"
```

Nesse caso, mantenha `~/.local/bin` antes de `/usr/local/bin` no PATH da sessão, ou execute `~/.local/bin/emwm-wallpaper-tool` diretamente.

## Utilização

Execute `emwm-wallpaper-tool`, selecione uma imagem, o posicionamento e a cor de fundo e clique em **Aplicar**. É possível aplicar a mesma configuração aos quatro workspaces, alterar apenas um ou escolher imagens e modos diferentes para cada um. As escolhas salvas são recuperadas ao abrir a ferramenta.

Modos disponíveis:

- **Ajustar à tela (sem recortar)**: mantém as proporções e exibe a imagem inteira, com margens quando necessário.
- **Preencher a tela (recortar)**: mantém as proporções e cobre toda a tela, recortando as bordas excedentes.
- **Esticar**: ocupa toda a tela, alterando as proporções da imagem.
- **Centralizar (tamanho original)**: centraliza sem redimensionar e recorta o que ultrapassar a tela.
- **Mosaico (tile)**: repete a imagem no tamanho original, a partir do canto superior esquerdo; é o padrão das configurações antigas.

A cor de fundo preenche as margens e as áreas transparentes. Por padrão, a imagem ocupa a área total da tela X11.

Para escolher fundos diferentes, clique em **Configurar fundo por monitor…** no workspace desejado (ou na opção de todos os workspaces). Selecione uma imagem, posicionamento e cor para cada monitor e confirme em **Usar estes fundos**. A gravação e a aplicação acontecem somente no botão **Aplicar** da janela principal. **Cancelar** descarta as mudanças do diálogo; **Usar imagem única** volta ao fundo que ocupa toda a tela.

Os monitores ativos são identificados pelo nome da saída (por exemplo, HDMI-1 e DP-1). A resolução, posição e rotação são detectadas via RandR. A opção **Automática** respeita as dimensões já orientadas pelo X11, inclusive monitores verticais, sem girar a imagem duas vezes. Também é possível girar a imagem 90°, 180° ou 270° dentro de cada monitor. Isso não altera a orientação física/configuração da saída no X11.

A ferramenta compõe um único PNG com os fundos nas posições de cada monitor, compatível com o mecanismo nativo do EMWM. Monitores espelhados compartilham pixels e são apresentados como uma única área. Escolhas de monitores desconectados são conservadas. Uma saída nova usa a imagem geral como alternativa, quando definida; caso contrário, a ferramenta pede uma imagem para ela. Na área ativa do EMWM Desktop, mudanças de resolução, rotação ou disposição dos monitores provocam uma recomposição automática. O comando `emwm-wallpaper-tool --refresh-layout` também executa essa atualização sem abrir a interface; escolhas de saídas desconectadas são preservadas. Em instalações independentes sem esse controlador, aplique novamente pela ferramenta.

No login seguinte, a sessão precisa carregar `~/.Xresources` **antes** de iniciar o EMWM. O `xmsession` do emwm-utils já faz isso; nesta instalação, a entrada de sessão usa `Exec=/usr/bin/xmsession`.

Para outras formas de iniciar o WM, execute `xrdb -merge "$HOME/.Xresources"` antes de iniciá-lo. O processamento de `#include` deve permanecer habilitado nessa leitura. Não é necessário executar a ferramenta no início da sessão.

Exemplo do arquivo gerenciado:

```text
Emwm*ws0*backdrop*image: "/home/usuario/.config/emwm/wallpapers/<hash>.png"
EmwmWallpaperTool.ws0.source: "/home/usuario/Imagens/fundo.jpg"
EmwmWallpaperTool.ws0.mode: fit
EmwmWallpaperTool.ws0.color: #000000
```

A ferramenta mantém o diretório `~/.config/emwm` usado pela versão anterior. Caminhos são absolutos e escapados para os recursos X e para o protocolo do WM. Nomes com espaços, acentos, aspas e barras invertidas são suportados; nomes com quebras de linha ou espaço no final são rejeitados porque não podem ser representados fielmente pelo parser do EMWM.

As cópias preparadas ficam em `~/.config/emwm/wallpapers/` e precisam continuar disponíveis para o EMWM restaurar os fundos. A imagem original não é modificada; mantenha-a disponível para mudar o posicionamento ou reaplicar em outra resolução. Configurações antigas ainda não reaplicadas continuam apontando diretamente para a imagem original.

A leitura das imagens usa os formatos disponíveis no GdkPixbuf instalado e respeita a orientação embutida da imagem. O EMWM exibe o PNG preparado usando o mecanismo nativo de fundos, sem serviços adicionais. Os nomes das cópias são derivados do conteúdo para evitar imagens antigas no cache do Motif; cópias idênticas são reutilizadas e as anteriores são preservadas para os backups de configuração.

## Migração e backups

Na primeira aplicação, se ainda não existe `backdrops.resources`, a ferramenta importa as escolhas do antigo `~/.config/emwm/apply-backdrops.sh`, sem executar o script. Depois da migração, o arquivo de recursos passa a ser a fonte das escolhas.

Para migrar as escolhas atuais sem abrir a interface nem mudar a imagem em exibição:

```bash
emwm-wallpaper-tool --migrate
```

Esse comando salva a configuração e atualiza os recursos da sessão X atual. O script antigo é preservado, mas deixa de ser atualizado e não é necessário. Remova eventuais chamadas a ele de arquivos de início de sessão para evitar que escolhas antigas substituam as novas.

O conteúdo existente de `~/.Xresources` é preservado e recebe apenas a inclusão do arquivo gerenciado. Antes da primeira alteração de um arquivo existente, uma cópia com o sufixo `.before-wallpaper-resources` é criada. As gravações usam arquivo temporário e substituição atômica; links simbólicos de arquivos existentes são preservados.

## Testes

`ctest` verifica migração, escape de caminhos, atualização parcial, preservação das configurações e backups, ausência de inclusões duplicadas, links simbólicos, persistência dos modos e cores, proporções, margens, recorte, mosaico, transparência, fundos por monitor, rotação manual, orientação vertical, posições com espaços entre monitores e persistência após desconexão.

O teste X11 opcional precisa de `Xvfb`, `xrdb`, `tellmwm` e um executável EMWM:

```bash
sudo apt install xvfb
EMWM_BINARY=/caminho/para/emwm cmake --build build --target test-x11
```

Ele usa um servidor X privado e um HOME temporário. Verifica pixels do fundo após aplicação imediata, reinício do WM e recarga dos recursos simulando um novo login, inclusive para caminhos com caracteres especiais, os cinco modos de posicionamento e uma composição com monitor horizontal e vertical. A composição com múltiplos monitores é simulada; a detecção RandR é verificada no monitor virtual do Xvfb. Não altera a sessão gráfica em uso.
