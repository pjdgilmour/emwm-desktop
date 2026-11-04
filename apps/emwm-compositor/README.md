# EMWM Compositor

Compositor X11 do emwm-desktop, sem processo picom/xcompmgr. C++17,
XComposite, XDamage, XFixes, XRender, Shape, RandR e OpenGL/GLX.

## Recursos

- Backends `auto`, `glx` e `xrender`; fallback XRender quando GLX não pode iniciar.
  Isso inclui falhas ao preparar as texturas das sombras. A seleção de FBConfig
  compara profundidade e máscaras de cor, permitindo visuais X11 equivalentes
  com identificadores diferentes, como os oferecidos pelo driver NVIDIA.
- Importação de pixmaps por `GLX_EXT_texture_from_pixmap`, sem leitura de pixels
  da janela pela CPU em cada quadro. Usa o driver GL da máquina; Mesa llvmpipe
  é um backend de software, mesmo quando GLX informa contexto direto.
- ARGB premultiplicado, opacidade `_NET_WM_WINDOW_OPACITY`, recorte Shape,
  janelas override-redirect e preservação das decorações Motif opacas.
- Sombra gaussiana gerada uma vez e desenhada em nove partes, com controles
  de raio, deslocamento e intensidade. Desktop/backdrops não recebem sombra.
- Animações de minimizar/restaurar/maximizar/desmaximizar, com imagem escalada
  e contornos Zoom, Twist e Flip adaptados do Window Maker. O giro/dobra afeta
  os contornos; o conteúdo mantém sua orientação, com zoom e transparência.
- Aparecimento gradual, recarga de preferências, troca de backend em execução,
  controle de quadros, sincronização vertical GLX quando disponível.
- Exclusividade por seleção X11, liberação dos recursos ao encerrar e sessão
  ainda utilizável sem composição.

## Arquitetura e limites

`main.cpp` acompanha janelas, danos, propriedades, empilhamento e transições.
`renderer.cpp` implementa os dois renderizadores; `effects.h` contém a geometria
pura dos contornos. As imagens dos frames são mantidas enquanto uma transição
precisa delas. Recursos GL são liberados antes de destruir o contexto.

O WM continua controlando geometria, foco, ícones e workspaces. O patch
`patches/emwm/0001-desktop-compositor.patch` publica `_EMWM_FRAME_CLIENT` nos
frames e envia `_EMWM_ANIMATION` para a root com SubstructureNotifyMask:
`[frame XID, tipo, icon-frame XID, fase, reservado]` (32 bits por campo).
Tipos: 1 minimizar, 2 restaurar do ícone, 3 maximizar, 4 desfazer maximização.
Fase 0 precede a mudança de estado; fase 1 a conclui. O compositor interpola
entre a geometria anterior e o destino real, sem adiar a mudança no WM.
Workspaces, criação inicial, retirada e superfícies de desktop não emitem essas
transições. Clientes transitórios conservam as regras existentes do WM.

A seleção `_NET_WM_CM_Sn` evita disputar a composição com outro processo.
O proprietário expõe `_EMWM_COMPOSITOR_BACKEND` e `_EMWM_LAST_ANIMATION`
para diagnóstico. Uma instância gerencia o X screen indicado por `DISPLAY`;
vários monitores RandR nesse screen compartilham a mesma composição.

Há repintura sob demanda com XDamage e limite de quadros. Cada quadro recompõe
a cena inteira, sem otimização de oclusão/dano parcial. Não há desfoque, HDR,
VRR, Vulkan, Wayland ou bypass de composição de fullscreen. VSync depende das
extensões GLX/driver e não é oferecido no backend XRender. Isso não equivale a
garantia de ausência de tearing, especialmente com taxas diferentes por monitor.
Não se reivindica paridade de desempenho ou compatibilidade com picom.

## Preferências

Use a aba **Efeitos e renderização** da ferramenta de aparência.
O arquivo é `$XDG_CONFIG_HOME/emwm-desktop/compositor.conf`, com os padrões em
`config/compositor.conf`. Alterações são detectadas em até um segundo;
SIGHUP/SIGUSR1 também solicitam recarga. `--backend` fixa um backend durante
aquela execução, portanto deve ser usado só para diagnóstico/testes se quiser
continuar alternando o backend pela interface.

## Compilação e testes

```sh
cmake -S apps/emwm-compositor -B build/compositor -DCMAKE_BUILD_TYPE=Release
cmake --build build/compositor -j4
ctest --test-dir build/compositor --output-on-failure
python3 tests/native-desktop.py --wm /caminho/do/emwm-com-patch \
    --compositor build/compositor/emwm-compositor
```

A suíte usa Xvfb, D-Bus, HOME e diretórios XDG temporários. Testa pixels de
conteúdo, danos, sombras, conteúdo durante minimização, três estilos, transições
do WM, desktop, workspaces, exclusividade, troca de backend, encerramento/reinício
e integração com PCManFM/GVfs. Os testes ARGB existentes do WM também aceitam
`EMWM_COMPOSITOR=/caminho/emwm-compositor`; o instalador usa essa opção.

Xvfb valida OpenGL com Mesa/software. Ainda é necessária validação de hardware
com Intel/AMD/NVIDIA, reprodução de vídeo, mudança de monitores e suspensão.

Licença do conjunto: GPL-3.0-only. A geometria derivada de Window Maker mantém
os créditos e GPL-2.0-or-later; veja `COPYING.WindowMaker` e `THIRD_PARTY.md`.
