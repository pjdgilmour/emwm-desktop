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

Autores originais do EMWM incluem The Open Group (Motif) e alx / fastestcode.org.
Os cabeçalhos originais são mantidos nos snapshots dos forks. `sources.lock.json`
identifica precisamente os commits usados. O tellmwm foi copiado dos fontes
locais da versão 1.2, sem alterações, preservando README, COPYING e cabeçalhos.
A ferramenta de fundos inclui as alterações de persistência desenvolvidas para
esta integração. Não são incluídos os artefatos de compilação da origem.

Bibliotecas e programas externos como Motif, picom, garcon, GTKmm, GNOME Keyring,
Polkit, Xorg e PipeWire são obtidos dos pacotes Debian e mantêm suas respectivas
licenças. Não são incorporados ao código deste repositório.

O pacote binário gerado inclui licenças, manifesto de versões e uma cópia dos
fontes e scripts usados em `usr/share/doc/emwm-desktop/sources.tar.xz`.
