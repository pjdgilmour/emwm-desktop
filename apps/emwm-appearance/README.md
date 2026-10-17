# Aparência do EMWM

Seleção de tema GTK, ícones, cursor/tamanho e paleta Motif opcional. A interface
GTK 3 usa o tema escolhido na prévia; nenhuma preferência é salva antes de
**Aplicar**. As decorações Motif conservam seu formato e relevo.

## Instalação independente (Debian)

Na raiz do repositório emwm-desktop:

```sh
sudo ./apps/emwm-appearance/install-system.sh
emwm-appearance
```

O script instala `python3-gi`, `python3-pil`, `gir1.2-gtk-3.0`, `xsettingsd`, `pkexec` e
`x11-xserver-utils`, os arquivos da ferramenta e seu auxiliar administrativo.
A ferramenta também integra o pacote construído pelo instalador do desktop.

Escolha tema, ícones e cursor, opcionalmente uma paleta Motif, e clique em
**Aplicar**. **Recarregar** procura novamente os temas instalados, descarta a
prévia e volta às preferências salvas.
A entrada **Aparência do EMWM** aparece nos menus XDG de configurações.

Temas com o mesmo nome aparecem uma única vez. A indicação **pessoal + sistema**
identifica os que existem nos dois locais: a sessão prioriza a cópia pessoal,
e a tela de login utiliza os arquivos do sistema. Não é necessário apagar
nenhuma cópia. A compatibilidade GTK do login é verificada na cópia do sistema.

## Tela de login

O gerenciador de login e o greeter são detectados a partir do serviço
`display-manager`, da configuração do LightDM e do arquivo `.desktop` do greeter.
Nesta versão, a aplicação de temas no login suporta **LightDM GTK Greeter**.
Outros gerenciadores/greeters são identificados e não são modificados.

Marque **Aplicar também à tela de login**. O Polkit solicitará autenticação
administrativa. O auxiliar altera somente `theme-name`, `icon-theme-name`,
`cursor-theme-name` e `cursor-theme-size` na seção `[greeter]` de
`/etc/lightdm/lightdm-gtk-greeter.conf`. Não altera autenticação, sessão, fundo,
lista de usuários ou indicadores; não reinicia o LightDM nem encerra a sessão.
A mudança aparece no próximo login. Cancelar a autenticação mantém o login
inalterado e conserva as preferências já aplicadas à sessão.

Para o login, os temas precisam estar em `/usr/share/themes`,
`/usr/local/share/themes`, `/usr/share/icons` ou `/usr/local/share/icons`.
Temas pessoais continuam utilizáveis na sessão, mas não são copiados para o
sistema automaticamente. O auxiliar não recebe caminhos arbitrários e valida
os nomes, o tamanho do cursor, a existência dos temas e o greeter detectado.
A interface inteira nunca é executada como root.

### Fundo da tela de login

Clique em **Fundo da tela de login…**, escolha uma imagem, o posicionamento e,
se necessário, a cor das margens. **Atualizar prévia** mostra o resultado sem
alterar a configuração. **Aplicar ao login** solicita autenticação e instala
uma cópia PNG em `/usr/share/backgrounds/emwm-desktop/`, legível pelo greeter.
A imagem original não é alterada e pode ser movida depois de aplicar.
Esta operação é independente do botão de aplicar temas e não muda o fundo da sessão.

- **Preencher (recortar)**: conserva proporções e cobre cada monitor, com recorte.
- **Esticar (expandir)**: ocupa cada monitor, podendo alterar proporções.
- **Ajustar (sem recortar)**: conserva proporções, com margens na cor selecionada.
- **Centralizar**: tamanho original centralizado, com recorte se for maior que a tela.
- **Mosaico (tile)**: repete a imagem a partir do canto superior esquerdo.

Preencher e esticar usam os modos nativos do LightDM GTK Greeter. Os demais
são preparados na resolução de referência, inicialmente detectada no monitor
principal e editável. Se a resolução do login for diferente, a composição será
redimensionada; reaplique usando a resolução desejada. Não há composição
independente por monitor nesta versão.

O auxiliar recebe somente dados PNG pelo stdin, com limites de tamanho e
resolução; ele não abre caminhos fornecidos pelo usuário. Valida e normaliza
a imagem e grava arquivos com permissão 0644. PNG, JPEG, WebP, BMP, TIFF e GIF
são aceitos pela interface (imagens animadas usam o primeiro quadro).
SVG não é aceito nesta versão.

A configuração define `background` e `user-background=false` em `[greeter]`
e nas seções de monitores existentes, preservando monitores marcados `#skip`.
Isso evita que o AccountsService substitua o fundo escolhido. Outras opções
do LightDM são preservadas; há backup antes da alteração, sem reiniciar o
gerenciador de login. Configurações por monitor presentes somente em arquivos
`conf.d` podem continuar prevalecendo e exigem revisão administrativa.
Imagens anteriores são conservadas para que os backups continuem utilizáveis.
As escolhas da interface são salvas após sucesso em `login-background.json`.

Ao atualizar a ferramenta, execute novamente `install-system.sh` para instalar
o novo auxiliar administrativo e a dependência Pillow (`python3-pil`).

## Persistência e atualização

- GTK 2: bloco identificado em `~/.gtkrc-2.0`.
- GTK 3/4: chaves de aparência em `settings.ini` no diretório XDG de configuração.
- Ícones Qt: `icon_theme` em configurações qt5ct/qt6ct já existentes.
- Cursor/Motif: inclusão identificada em `~/.Xresources`.
- Sessão: bloco identificado em `~/.Xprofile`, ativo apenas no EMWM.
- Dados próprios: `$XDG_CONFIG_HOME/emwm-desktop/appearance/` (padrão `~/.config`).
- Backups: `$XDG_STATE_HOME/emwm-desktop/appearance-backups/`, com manifesto
  indicando o destino original de cada arquivo; padrão `~/.local/state`.
- Backups administrativos: `/var/backups/emwm-appearance/`.

Arquivos alheios, comentários, fontes, atalhos, monitor e papéis de parede são
preservados. Escritas usam substituição atômica e preservam permissões/links.
Em erro durante a gravação da sessão, os arquivos já gravados são restaurados.

`xsettingsd` atualiza clientes GTK compatíveis sem daemon de um desktop completo.
A ferramenta mantém uma instância por DISPLAY e recarrega somente sua própria
instância. Não substitui um proprietário XSETTINGS existente de XFCE/GNOME/etc.
Preferências GNOME equivalentes são gravadas via GSettings quando disponíveis.

A primeira aplicação remove o efeito de `GTK_THEME` no próximo início da sessão,
por um bloco `unset GTK_THEME` posterior às configurações existentes. Processos
já iniciados com essa variável ainda podem impor o tema antigo; nesse caso,
saia e entre novamente uma vez. As cores do WM precisam de reinício do EMWM;
outros aplicativos Motif e alguns cursores precisam ser reabertos.

## Limites de compatibilidade

GTK 2, GTK 3, GTK 4, Qt, libadwaita e Motif têm mecanismos distintos. Não existe
um arquivo GTK CSS universal para todos. O seletor informa as versões presentes
no tema. O Irixium deste sistema oferece GTK 3: não inclui automaticamente um
tema equivalente para GTK 2/4 ou Qt. Aplicativos libadwaita e aplicativos com
estilo próprio podem ignorar a seleção. A ferramenta não injeta CSS global.

A paleta **Manter as cores atuais** deixa os recursos Motif pessoais em vigor;
**IRIX clássico** e **Motif cinza** oferecem cores opcionais sem mudar o desenho
das molduras. Não é feita conversão automática de CSS GTK para Motif.

## Desenvolvimento e testes

```sh
python3 -m unittest discover -s apps/emwm-appearance/tests -v
apps/emwm-appearance/tests/run-x11.sh
python3 apps/emwm-appearance/gui.py --list
```

O teste gráfico requer Xvfb e xsettingsd. Usa Adwaita por padrão;
`APPEARANCE_TEST_THEME=Irixium` testa Irixium quando instalado. Usa HOME,
servidor X e D-Bus privados e GSettings em memória. Testa prévia sem gravação,
aplicação, preservação de configurações e leitura real em outro cliente GTK.
O auxiliar de login é testado numa árvore temporária, sem modificar o LightDM
real. O login real depende da instalação e autenticação administrativa.

Referências: [Gtk.Settings](https://docs.gtk.org/gtk3/class.Settings.html),
[xsettingsd](https://github.com/derat/xsettingsd) e
[configuração do LightDM GTK Greeter](https://github.com/Xubuntu/lightdm-gtk-greeter/blob/master/data/lightdm-gtk-greeter.conf).

Licença: GPL-3.0-only, conforme LICENSE do projeto emwm-desktop.
