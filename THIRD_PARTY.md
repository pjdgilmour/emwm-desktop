# Créditos, fontes e licenças

EMWM Desktop integra projetos independentes. O nome deste repositório não
transfere a autoria nem substitui as licenças dos componentes.

| Componente | Origem | Licença / avisos |
| --- | --- | --- |
| EMWM | https://github.com/alx210/emwm | Preservar `COPYING` (texto LGPL 2.1) e os avisos específicos dos arquivos Motif/EMWM. |
| Alterações do WM | https://github.com/pjdgilmour/emwm | Botão de fechar clássico e preservação de ARGB; seguem os avisos do projeto. |
| emwm-utils | https://github.com/alx210/emwm-utils | MIT, copyright alx@fastestcode.org. |
| Alterações dos utilitários | https://github.com/pjdgilmour/emwm-utils | Menus XDG e bandeja xmtray; MIT. |
| tellmwm 1.2 | https://fastestcode.org/emwm.html | Licença original em `third_party/tellmwm/COPYING`: redistribuição/modificação com crédito ao autor e link à origem. |
| emwm-wallpaper-tool | https://github.com/pjdgilmour/minhasapps | Copiado do projeto do usuário, cujo repositório contém GPL versão 3; cópia da licença em `apps/emwm-wallpaper-tool/COPYING`. |
| Scripts de integração deste repositório | pjdgilmour | GPL-3.0-only; ver `LICENSE`. |
| EMWM Compositor e integração do desktop | pjdgilmour / emwm-desktop | GPL-3.0-only; efeitos contêm geometria adaptada do Window Maker GPL-2.0-or-later, combinada sob GPL 3. |
| PCManFM 1.4.0 | https://github.com/lxde/pcmanfm / fonte original Debian | GPL-2.0-or-later; cópia privada com correção de encerramento de arrasto em `patches/pcmanfm/`. SHA-256 em `sources.lock.json`; fontes e COPYING incluídos no pacote. |
| Geometria Zoom/Twist/Flip | Window Maker, `src/actions.c`, commit `558e67f30331eb03955d7bf3bf4a55c8520c64b6`, https://github.com/window-maker/wmaker | Alfredo K. Kojima (1997–2003), Dan Pascu (1998–2003), Window Maker Team (2014–2026). GPL-2.0-or-later; texto original em `apps/emwm-compositor/COPYING.WindowMaker`. |

Autores originais do EMWM incluem The Open Group (Motif) e alx / fastestcode.org.
Os cabeçalhos originais são mantidos nos snapshots dos forks. `sources.lock.json`
identifica precisamente os commits de base usados. As alterações locais em
`patches/emwm-utils/`, incluindo a bandeja integrada ao Toolchest, mantêm a
licença MIT dos utilitários. O tellmwm foi copiado dos fontes
locais da versão 1.2, sem alterações, preservando README, COPYING e cabeçalhos.
A ferramenta de fundos inclui as alterações de persistência desenvolvidas para
esta integração. Não são incluídos os artefatos de compilação da origem.

Bibliotecas e programas externos como Motif, OpenGL/GLX, garcon, GTKmm,
GVfs, GNOME Keyring, Polkit, Xorg e PipeWire são obtidos dos pacotes Debian e mantêm suas respectivas
licenças. Não são incorporados ao código deste repositório.
O executável privado do PCManFM usa as bibliotecas, os dados e as traduções do
pacote Debian; a instalação global de PCManFM não é substituída.
O compositor não incorpora nem executa picom ou xcompmgr. Os efeitos usam
contornos inspirados/adaptados do Window Maker com interpolação temporal própria;
o loop bloqueante e os grabs de servidor das animações originais não são usados.

O pacote binário gerado inclui licenças, manifesto de versões e uma cópia dos
fontes e scripts usados em `usr/share/doc/emwm-desktop/sources.tar.xz`.
