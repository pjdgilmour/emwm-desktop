# Migração do EMWM Desktop para Rust

O pacote Debian 13/X11 combina os programas Rust desta distribuição com as revisões de EMWM, emwm-utils e PCManFM fixadas em `sources.lock.json`. Os patches vinculam cada módulo portado ao executável que o utiliza. O código C do tellmwm 1.2 em `third_party/` serve de referência.

| Componente | Código Rust | Como verificar |
| --- | --- | --- |
| Aparência e login | `appearance/app` | Testes de preferências, gravação com backup, diálogo GTK e auxiliar LightDM com raiz temporária |
| Gradientes da barra de título | `gradient-rs` | `rust-anything gate rust/gradient-rs` |
| Compositor | `compositor` | CTest, comparação de CLI, pixels em Xvfb com GLX e XRender, troca em execução e fallback |
| Fundos | `wallpaper` | CTest e alvos CMake `test-cli-parity` e `test-x11`, recomposição após hotplug |
| Ícones | `desktop/src/bin/emwm-desktop-icons.rs` | Cargo, testes de arrasto, mudança de workspace, monitor primário, lixeira e reinício |
| Sessão | `desktop/src/bin/emwm-session.rs` | Testes de recursos, perfil, processos filhos e inicialização do pacote extraído |
| Instalador | `desktop/src/bin/emwm-install.rs` | Replay da CLI em `desktop/rust/app-cases` e `./install.sh --build-only --test` |
| EMWM, monitores, escolha da tela para encaixe e gradientes XRender | `emwm` | Testes Xinerama, `make test-tiling` e `tests/title-gradient.py` |
| Cliente de workspaces tellmwm | `tellmwm` | Replay da CLI e `tests/tellmwm.sh` em dois displays privados |
| Menu do Toolchest | `emwm-utils` | Replay da leitura de toolboxrc e testes Motif |
| Bandeja integrada e xmtray independente | `tray` | Clientes GTK/Qt e `tests/run-toolchest-tray.sh` |
| Entrada xinit/xdm xmsession | `desktop/src/bin/xmsession.rs` | Rotação do log, leitura de recursos e perfil POSIX em `tests/test_integration.py` |
| Desktop do PCManFM | `pcmanfm` | Grade, arrasto, ancoragem, composição e persistência dos ícones por saída em Xvfb |
| Editor Troca Cor | `trocacor` | `cargo test --locked --manifest-path rust/trocacor/Cargo.toml` |

Os testes usam os formatos e caminhos existentes em Xresources e XDG. Os arquivos de perfil e de sessão aceitam comandos POSIX, por isso o supervisor usa `/bin/sh` ao carregá-los. Os binários do desktop usam as bibliotecas de sistema para GTK e X11. A troca da sessão instalada depende da instalação explícita do pacote.

Continuam em C o núcleo de eventos e gerenciamento de clientes do EMWM, as partes Motif/XDG do Toolchest, o gerenciador de sessão xmsm e a interface de arquivos do PCManFM.
