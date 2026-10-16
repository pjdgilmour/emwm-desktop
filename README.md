# EMWM Desktop

Ambiente X11 clássico baseado no **Enhanced Motif Window Manager**, com botão de
fechar, transparência por aplicativo via picom, menus XDG automáticos no
Toolchest, bandeja Motif `xmtray` e configuração persistente de papéis de parede.

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

O instalador instala as dependências de compilação usando apt, obtém os commits
fixados em `sources.lock.json`, compila como usuário normal, executa os testes e
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

Esses caminhos permitem coexistir com instalações anteriores do EMWM. O pacote
não substitui `/usr/bin/emwm`, `/usr/local/bin/emwm` nem as configurações do XFCE.
Arquivos pessoais `.profile`, `.Xprofile`, `.Xresources`, `.emwmrc` e `.toolboxrc`
permanecem intactos. A sessão respeita esses arquivos quando existem. Para testar
os padrões em uma máquina já personalizada, use uma conta de usuário nova.

O `xmsm` mantém a instalação setuid root usada pelo projeto original para bloqueio
de tela e operações da sessão. Os demais componentes rodam sem esse privilégio.
O comando de suspensão é compilado como `systemctl suspend` no Debian.

Dependências de execução incluem Motif, garcon/GLib, GTKmm para a ferramenta de
fundos, Xorg, picom, agente Polkit MATE, GNOME Keyring, XTerm e PCManFM.
NetworkManager Applet, PipeWire e pavucontrol são recomendações do pacote.
Não inclui XFCE, AnyDesk, drivers NVIDIA, ferramentas Blackmagic ou temas de terceiros.

## Preferências da sessão

Para personalizar, copie o exemplo apenas se ainda não houver uma configuração:

```sh
mkdir -p ~/.config/emwm-desktop
cp -n /usr/share/emwm-desktop/session.conf ~/.config/emwm-desktop/session.conf
```

Edite esse arquivo para desabilitar compositor, bandeja, applet de rede, chaveiro
ou agente Polkit. O compositor usa `xrender` como padrão portátil; altere
`EMWM_DESKTOP_PICOM_BACKEND=glx` se preferir e seu driver suportar.
Nenhuma saída de monitor, rotação, teclado ou tema GTK particular é imposta.
A sessão lê seu `.Xprofile`, depois `session.conf`, e carrega os recursos X antes
de iniciar o WM e os utilitários. Logs ficam em
`~/.local/state/emwm-desktop/session.log` (ou em `XDG_STATE_HOME`).

Para selecionar o fundo, use **Ambiente → Papel de parede** no Toolchest. O caminho
da imagem precisa continuar disponível no próximo login. A ferramenta cria
`~/.config/emwm/backdrops.resources` e uma inclusão em `.Xresources`, preservando
seu conteúdo e criando backup antes da alteração. A imagem é apresentada pelo
modo nativo do EMWM; não há redimensionamento automático.

O `xmtray` implementa bandeja X11/XEmbed, com menus nativos dos aplicativos.
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
- `apps/emwm-wallpaper-tool/`: ferramenta de fundos.
- `third_party/tellmwm/`: cópia do tellmwm 1.2, com licença original.
- `tests/`: verificações do pacote e da configuração.

```sh
python3 -m unittest discover -s tests -v
```

Os testes `--test` executam os testes do WM, transparência, menus, bandeja e
persistência dos fundos em servidores Xvfb privados. Também verificam o conteúdo
do pacote e a inicialização da sessão completa extraída do `.deb`. Não substituem o WM da
sessão em uso. O pacote inclui `sources.tar.xz` com os fontes dos binários e os
scripts usados na compilação, além das licenças correspondentes.
