# emwm-wallpaper-tool

Aplicativo gráfico em C++17 com GTKmm 3 para configurar papéis de parede dos quatro workspaces do EMWM. A interface e a apresentação das imagens pelo WM permanecem as mesmas.

Ao clicar em **Aplicar**, a ferramenta salva as escolhas em `~/.config/emwm/backdrops.resources`, inclui esse arquivo em `~/.Xresources` e aplica a imagem imediatamente com `tellmwm`. Também atualiza o banco de recursos do servidor X com `xrdb -merge`, para preservar a escolha ao reiniciar apenas o WM.

O EMWM restaura as imagens diretamente de seus recursos nativos no início da sessão. Não há script de reaplicação, espera com `sleep`, serviço adicional ou mudança no código do WM. Alterar apenas um workspace preserva os demais.

## Dependências e compilação

```bash
sudo apt install build-essential cmake pkg-config libgtkmm-3.0-dev libx11-dev x11-xserver-utils
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j2
ctest --test-dir build --output-on-failure
sudo cmake --install build
```

Dependências de execução: GTKmm 3, libX11, `xrdb`, `xrandr` (resolução exibida na interface) e `tellmwm` do EMWM (aplicação imediata). A falta de `xrdb` ou uma falha no comando é informada; a configuração salva continua disponível para o próximo login.

Para instalar somente para seu usuário (sem senha de administrador):

```bash
cmake --install build --prefix "$HOME/.local"
```

Nesse caso, mantenha `~/.local/bin` antes de `/usr/local/bin` no PATH da sessão, ou execute `~/.local/bin/emwm-wallpaper-tool` diretamente.

## Utilização

Execute `emwm-wallpaper-tool`, selecione uma imagem e clique em **Aplicar**. No login seguinte, a sessão precisa carregar `~/.Xresources` **antes** de iniciar o EMWM. O `xmsession` do emwm-utils já faz isso; nesta instalação, a entrada de sessão usa `Exec=/usr/bin/xmsession`.

Para outras formas de iniciar o WM, execute `xrdb -merge "$HOME/.Xresources"` antes de iniciá-lo. O processamento de `#include` deve permanecer habilitado nessa leitura. Não é necessário executar a ferramenta no início da sessão.

Exemplo do arquivo gerenciado:

```text
Emwm*ws0*backdrop*image: "/home/usuario/Imagens/fundo.jpg"
Emwm*ws1*backdrop*image: "/home/usuario/Imagens/outro fundo.png"
```

A ferramenta mantém o diretório `~/.config/emwm` usado pela versão anterior. Caminhos são absolutos e escapados para os recursos X e para o protocolo do WM. Nomes com espaços, acentos, aspas e barras invertidas são suportados; nomes com quebras de linha ou espaço no final são rejeitados porque não podem ser representados fielmente pelo parser do EMWM.

A imagem deve continuar disponível no caminho salvo no início da sessão. O formato precisa ser suportado pela biblioteca Motif instalada. O EMWM continua usando seu modo nativo de apresentação (repetição da imagem); esta alteração não acrescenta redimensionamento ou recorte.

## Migração e backups

Na primeira aplicação, se ainda não existe `backdrops.resources`, a ferramenta importa as escolhas do antigo `~/.config/emwm/apply-backdrops.sh`, sem executar o script. Depois da migração, o arquivo de recursos passa a ser a fonte das escolhas.

Para migrar as escolhas atuais sem abrir a interface nem mudar a imagem em exibição:

```bash
emwm-wallpaper-tool --migrate
```

Esse comando salva a configuração e atualiza os recursos da sessão X atual. O script antigo é preservado, mas deixa de ser atualizado e não é necessário. Remova eventuais chamadas a ele de arquivos de início de sessão para evitar que escolhas antigas substituam as novas.

O conteúdo existente de `~/.Xresources` é preservado e recebe apenas a inclusão do arquivo gerenciado. Antes da primeira alteração de um arquivo existente, uma cópia com o sufixo `.before-wallpaper-resources` é criada. As gravações usam arquivo temporário e substituição atômica; links simbólicos de arquivos existentes são preservados.

## Testes

`ctest` verifica migração, escape de caminhos, atualização parcial, preservação das configurações e backups, ausência de inclusões duplicadas e links simbólicos.

O teste X11 opcional precisa de `Xvfb`, `xrdb`, `tellmwm` e um executável EMWM:

```bash
sudo apt install xvfb
EMWM_BINARY=/caminho/para/emwm cmake --build build --target test-x11
```

Ele usa um servidor X privado e um HOME temporário. Verifica pixels do fundo após aplicação imediata, reinício do WM e recarga dos recursos simulando um novo login, inclusive para caminhos com caracteres especiais. Não altera a sessão gráfica em uso.
