# EMWM Desktop

Ambiente X11 clássico baseado no **Enhanced Motif Window Manager**, com botão de
fechar, compositor X11 próprio com OpenGL/XRender, sombras e animações,
desktop com arquivos e lixeira, menus XDG automáticos no Toolchest,
bandeja Motif `xmtray` e configuração persistente de papéis de parede.

Este é um projeto de integração mantido por **pjdgilmour**, baseado no trabalho
original de **alx / fastestcode.org** e no Motif. Os componentes preservam suas
licenças e créditos; consulte [THIRD_PARTY.md](THIRD_PARTY.md).

- Fork do WM: <https://github.com/pjdgilmour/emwm>
- Fork dos utilitários: <https://github.com/pjdgilmour/emwm-utils>
- Originais: <https://github.com/alx210/emwm> e <https://github.com/alx210/emwm-utils>

## Instalar no Debian 13

Requisitos iniciais: Debian 13, usuário comum com acesso a `sudo`, Python 3 e Git.
Se necessário, instale `sudo git python3 ca-certificates` como administrador.
A instalação precisa de acesso à internet e não deve ser executada como root.

```sh
git clone https://github.com/pjdgilmour/emwm-desktop.git
cd emwm-desktop
./install.sh --test
```

O instalador instala as dependências de compilação usando apt, obtém os fontes
fixados em `sources.lock.json` (commits e arquivos com SHA-256), compila como usuário normal, executa os testes e
gera um pacote `.deb`. Por fim, instala o pacote com apt, que resolve também as
dependências de execução. `--yes` aceita as confirmações do apt; omita para revisar.
Sem `--test`, mantém os testes básicos da ferramenta de fundos, mas não executa
os testes gráficos completos.

Selecione **EMWM Desktop** na tela de login. O instalador não encerra a sessão
atual e não troca automaticamente o ambiente ou o gerenciador de login padrão.
Em uma instalação sem gerenciador de login, o pacote recomenda LightDM; se houver
outro provedor de `x-display-manager` instalado, ele pode ser mantido.
Também é possível usar `startx /usr/bin/emwm-desktop-session` a partir de um TTY.

## O que é instalado

- Executáveis privados: `/usr/lib/emwm-desktop/bin/`.
- Recursos Motif, menus e exemplos: `/usr/share/emwm-desktop/`.
- Inicializador da sessão: `/usr/bin/emwm-desktop-session`.
- Entrada do login: `/usr/share/xsessions/emwm-desktop.desktop`.
- Licenças e cópias dos fontes: `/usr/share/doc/emwm-desktop/`.

Há uma única entrada EMWM na tela de login: **EMWM Desktop**. A instalação
desvia a entrada antiga `emwm.desktop` para
`/usr/share/emwm-desktop/legacy/emwm.desktop` usando `dpkg-divert`; ao remover
o pacote, a entrada antiga é restaurada. Os binários antigos `/usr/bin/emwm`
e `/usr/local/bin/emwm` e as configurações do XFCE não são substituídos.
O PATH da sessão prioriza também o lançador atualizado da ferramenta de aparência,
para que uma instalação antiga em `~/.local` não oculte as opções de composição.
Arquivos pessoais `.profile`, `.Xprofile`, `.Xresources`, `.emwmrc` e `.toolboxrc`
permanecem intactos. A sessão respeita esses arquivos quando existem. Para testar
os padrões em uma máquina já personalizada, use uma conta de usuário nova.

O `xmsm` mantém a instalação setuid root usada pelo projeto original para bloqueio
de tela e operações da sessão. Os demais componentes rodam sem esse privilégio.
O comando de suspensão é compilado como `systemctl suspend` no Debian.

Dependências de execução incluem Motif, garcon/GLib, GTKmm para a ferramenta de
fundos, Xorg, bibliotecas OpenGL/GLX e de composição X11, agente Polkit MATE,
GNOME Keyring, XTerm, PCManFM, GVfs e xdg-user-dirs. Picom e xcompmgr não são
necessários nem iniciados por esta versão.
NetworkManager Applet, PipeWire e pavucontrol são recomendações do pacote.
Não inclui XFCE, AnyDesk, drivers NVIDIA, ferramentas Blackmagic ou temas de terceiros.

## Preferências da sessão

Para personalizar, copie o exemplo apenas se ainda não houver uma configuração:

```sh
mkdir -p ~/.config/emwm-desktop
cp -n /usr/share/emwm-desktop/session.conf ~/.config/emwm-desktop/session.conf
```

Edite esse arquivo para desabilitar compositor, ícones do desktop, bandeja,
applet de rede, chaveiro ou agente Polkit. A renderização e os efeitos são
selecionados em **Ambiente → Aparência → Composição, sombras e animações**.
Nenhuma saída de monitor, rotação, teclado ou tema GTK particular é imposta.
A sessão lê seu `.Xprofile`, depois `session.conf`, e carrega os recursos X antes
de iniciar o WM e os utilitários. Logs ficam em
`~/.local/state/emwm-desktop/session.log` (ou em `XDG_STATE_HOME`).

### Compositor próprio e efeitos

O `emwm-compositor` pertence ao projeto e roda em processo separado do WM.
O modo **Automática** tenta OpenGL/GLX com texturas de pixmaps X11; se esse
backend não puder ser iniciado, usa XRender. OpenGL pode utilizar a GPU com
um driver compatível. Em servidores virtuais pode usar renderização por software;
o nome do renderizador e a disponibilidade de sincronização vertical aparecem
no log da sessão. Vulkan e Wayland não fazem parte desta versão.

Na ferramenta de aparência é possível selecionar:

- **Zoom** (escala), **Twist** (giro do conteúdo) ou **Flip** (achatamento e
  inversão vertical), inspirados no Window Maker, para minimizar, restaurar,
  maximizar e desfazer a maximização. Os efeitos transformam a imagem da janela
  em OpenGL e XRender, além dos contornos.
- Duração das animações e do aparecimento gradual das janelas.
- Sombras suaves, intensidade, raio e deslocamento.
- Backend, sincronização vertical OpenGL e limite de quadros por segundo.

As escolhas só são gravadas ao clicar **Aplicar**. O compositor detecta as
alterações em até um segundo, inclusive a troca do backend, sem reiniciar o WM.
A configuração fica em `$XDG_CONFIG_HOME/emwm-desktop/compositor.conf`
(normalmente `~/.config/emwm-desktop/compositor.conf`); há um exemplo em
`/usr/share/emwm-desktop/compositor.conf`. Os efeitos não bloqueiam o loop de
eventos do WM. Trocas de workspace não são tratadas como minimização.

`EMWM_DESKTOP_COMPOSITOR=0` em `session.conf` desativa o componente no próximo
login. A antiga opção `EMWM_DESKTOP_PICOM_BACKEND` não é mais usada. Caso seu
`.Xprofile` inicie outro compositor, remova essa inicialização para usar o novo:
o EMWM Compositor detecta um proprietário existente e não o substitui.

Esta implementação oferece composição, transparência, sombras e os efeitos
descritos; não implementa desfoque, HDR, VRR ou apresentação direta de fullscreen.
XRender não oferece a opção de VSync do backend OpenGL. Os testes com Xvfb/Mesa
validam o funcionamento, mas desempenho, tearing e suspensão precisam ser
avaliados com os drivers e monitores reais. Consulte
[os detalhes técnicos e testes](apps/emwm-compositor/README.md).

### Arquivos, ícones e lixeira no desktop

Com a área ativa, o PCManFM desenha o fundo em **todos** os monitores, usando
o recorte correspondente da imagem composta pela ferramenta de papel de parede.
Isso inclui os monitores sem ícones. O EMWM oculta seus backdrops nativos
enquanto as superfícies do desktop estão presentes e volta a exibi-los quando
elas são encerradas. A mesma configuração por workspace continua sendo usada
pelos dois caminhos; não é preciso apagar ou desabilitar os fundos salvos.
Os pixmaps recortados de cada monitor não são publicados como fundo global do
X11, evitando deslocamentos e referências a pixmaps liberados.

Em **Aparência → Área de trabalho** (ou `emwm-appearance --desktop`), escolha o
**Monitor dos ícones** e a ancoragem **Esquerda/Direita**. Por padrão, arquivos,
lixeira e volumes aparecem somente no principal informado pelo X11/driver.
Também é possível escolher uma saída pelo nome, como `DP-4`; isso define o
monitor dos ícones sem mudar a configuração de vídeo do sistema. Se a saída
escolhida for desconectada, os ícones passam ao principal disponível (ou primeiro
ativo) e voltam à saída escolhida quando ela for reconectada. Os demais monitores
mantêm o fundo de tela. A preferência é salva em
`~/.config/emwm-desktop/desktop.conf`, respeitando `XDG_CONFIG_HOME`.

A aplicação é imediata, após terminar qualquer arrasto em andamento, sem
reiniciar a sessão ou fechar janelas do gerenciador de arquivos. A troca de lado
espelha horizontalmente as posições manuais, mantendo a altura; os ícones
automáticos começam na borda escolhida. Nas preferências da área de trabalho,
o botão **Monitor e alinhamento dos ícones…** também abre essa aba. O botão
**Configurar papel de parede…** dá acesso
à configuração dos fundos por workspace e monitor.

O `emwm-desktop-icons` inicia o gerenciador de desktop do PCManFM com o perfil
`emwm-desktop`. Ele mostra a pasta indicada por `xdg-user-dir DESKTOP`, incluindo
pastas localizadas, e oferece seleção, arrastar e soltar, copiar, mover, renomear,
abrir arquivos e organizar ícones. O ícone de lixeira usa GVfs e o padrão
freedesktop, com abertura, restauração e esvaziamento pelo PCManFM.

O pacote inclui um executável privado do PCManFM 1.4.0 com correção de `drag-end`:
ao soltar ou cancelar um arrasto, o ícone volta a ser desenhado imediatamente,
sem depender de outro clique. A instalação global `/usr/bin/pcmanfm` é mantida;
o componente privado utiliza as bibliotecas, traduções e dados do Debian.

O EMWM trata essas superfícies sem molduras, abaixo das aplicações e presentes
em todos os workspaces. Os arquivos são compartilhados entre os workspaces;
os ícones de aplicações minimizadas continuam pertencendo ao WM. Evite colocar
arquivos na mesma região usada por esses ícones, ou habilite a Icon Box do EMWM.
O menu de arquivos aparece com o botão direito; os comandos do ambiente continuam
disponíveis no Toolchest e pelos atalhos do EMWM.

Os fundos preparados pela ferramenta de papel de parede são sincronizados com
o desktop ativo ao aplicar uma imagem e ao trocar de workspace. As preferências
e posições dos ícones ficam em `~/.config/pcmanfm/emwm-desktop/`, respeitando
`XDG_CONFIG_HOME`. Perfis preexistentes são preservados. Como o PCManFM mantém
uma instância por display, deixe a sessão iniciar o desktop antes de abrir suas
janelas de arquivos. Não o inicie também no `.Xprofile` com outro perfil.
Use `EMWM_DESKTOP_ICONS=0` para desativar o desktop ativo no próximo login.

### Fundos, bandeja e serviços

Para selecionar o fundo, use **Ambiente → Papel de parede** no Toolchest. O caminho
da imagem original é preservado para futuros ajustes. A ferramenta cria
`~/.config/emwm/backdrops.resources` e uma inclusão em `.Xresources`, preservando
seu conteúdo e criando backup antes da alteração. Há opções de ajustar sem recorte,
preencher com recorte, esticar, centralizar e mosaico, com cor de fundo por workspace.
Em **Configurar fundo por monitor…**, escolha imagens e modos independentes para
cada saída. A orientação vertical é detectada via RandR; há também giro manual
da imagem por monitor.
As cópias preparadas ficam em `~/.config/emwm/wallpapers/` e são restauradas pelo
EMWM no login. Após mudar a resolução ou a disposição dos monitores, aplique o
fundo novamente na ferramenta.

A bandeja X11/XEmbed fica integrada ao Toolchest, abaixo da data/hora, com
quatro ícones por linha. A altura se ajusta automaticamente quando os ícones
entram, saem ou ficam ocultos. Os menus nativos dos aplicativos são preservados.
Cada linha fica centralizada, sem reservar espaço para ícones ausentes, e a
área da bandeja desaparece quando não há ícones visíveis.
Para desativá-la, use `EMWM_DESKTOP_TRAY=0` em `session.conf`.
O executável `xmtray` continua disponível para uso avulso, mas a sessão não
abre uma janela de bandeja separada.
Aplicativos exclusivamente StatusNotifier/AppIndicator sem alternativa XEmbed
não são suportados nesta versão. Não há daemon de notificações integrado.

O chaveiro usa Secret Service. Caso o VS Code não reconheça o ambiente, configure
`"password-store": "gnome-libsecret"` no arquivo de argumentos de runtime do VS Code
(comando **Preferences: Configure Runtime Arguments**) e reinicie o aplicativo.
O instalador não altera configurações particulares de aplicativos.

## Gerar sem instalar

Com as dependências de compilação já presentes:

```sh
./install.sh --build-only --test
```

O pacote e seu SHA-256 aparecem em `dist/`. O modo `--build-only` não usa sudo,
não chama apt e não instala nem modifica a sessão atual.

Para instalar apenas as dependências de compilação/teste:

```sh
./install.sh --dependencies-only --test
```

Para desenvolver antes de publicar os commits dos forks:

```sh
./install.sh --build-only --test --sources /diretorio/que/contem/os/repositorios
```

Esse diretório deve conter `emwm/` e `emwm-utils/`. São extraídos os **commits
fixados**, sem copiar artefatos de compilação ou alterações ainda não commitadas.
Depois de modificar um fork, atualize `sources.lock.json` com o novo SHA completo.
O fonte do PCManFM é obtido separadamente do arquivo Debian fixado por SHA-256
no mesmo manifesto e armazenado em `build/cache/`, inclusive com `--sources`.
Os patches em `patches/<componente>/` são aplicados em ordem sobre esses commits
antes da compilação. A integração da bandeja ao Toolchest está nesse diretório;
o pacote inclui tanto os patches quanto os fontes já modificados.
Publique os commits nos forks **antes** de publicar uma versão do instalador que
os referencia. O instalador não segue automaticamente a ponta de uma branch.

Os binários são recompilados para a arquitetura local. A primeira versão foi
compilada e testada em Debian 13 amd64; uma instalação completa em uma máquina
nova deve ser validada antes de declarar suporte a outros ambientes/arquiteturas.

## Remover

```sh
sudo apt remove emwm-desktop
```

Arquivos pessoais e imagens escolhidas permanecem. Dependências podem ser
avaliadas separadamente com `apt autoremove`; o instalador não remove outros
ambientes gráficos nem troca preferências globais.

## Estrutura e testes

- `scripts/install.py`: obtenção dos fontes, compilação e montagem do pacote.
- `session/`: inicializador independente.
- `config/`: padrões e exemplos genéricos.
- `patches/`: alterações locais aplicadas aos snapshots fixados dos componentes.
- `apps/emwm-wallpaper-tool/`: ferramenta de fundos.
- `apps/emwm-compositor/`: compositor X11 próprio, OpenGL/XRender e efeitos.
- `apps/emwm-desktop-icons/`: integração do desktop PCManFM, fundos e lixeira.
- `third_party/tellmwm/`: cópia do tellmwm 1.2, com licença original.
- `tests/`: verificações do pacote e da configuração.

```sh
python3 -m unittest discover -s tests -v
```

Os testes `--test` executam os testes do WM, transparência com o compositor próprio, menus, bandeja e
persistência dos fundos em servidores Xvfb privados. Também verificam o conteúdo
do pacote e a inicialização da sessão completa extraída do `.deb`. Não substituem o WM da
sessão em uso. O pacote inclui `sources.tar.xz` com os fontes dos binários e os
scripts usados na compilação, além das licenças correspondentes.
Os testes adicionais verificam pixels de conteúdo/sombra, as três animações,
empilhamento e presença do desktop nos workspaces, troca de backend em execução,
exclusividade do compositor, lixeira e reinício. Usam HOME, DISPLAY e D-Bus
privados; não substituem o compositor da sessão em uso.

## Aparência do ambiente e do login

A ferramenta **Aparência do EMWM** permite selecionar tema GTK, ícones, cursor,
tamanho do cursor e cores Motif opcionais. As cores de fundo, texto, decoração
ativa/inativa e seleção de ícones podem ser editadas com base no `trocacor.cpp`.
A prévia não grava preferências; a aplicação preserva os backups.
Há integração com LightDM GTK Greeter mediante autenticação administrativa,
sem reiniciar o gerenciador de login. Outros gerenciadores são detectados, mas
não recebem alterações. Temas do login precisam estar instalados globalmente.

```sh
# Instalar apenas a ferramenta em uma instalação EMWM já existente:
sudo ./apps/emwm-appearance/install-system.sh
emwm-appearance
```

O pacote completo também inclui a ferramenta e suas dependências. Consulte
[os detalhes de uso, backups e compatibilidade](apps/emwm-appearance/README.md).
