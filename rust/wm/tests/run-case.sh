#!/bin/sh
# The window manager contract includes geometry and X11 events. The upstream
# probes send real keyboard and pointer input to their own Xvfb server.
set -eu
: "${EMWM_PARITY_TREE:?patched EMWM source tree required}"
: "${EMWM_BINARY:?absolute path to the window manager required}"
case "$1" in
    decoration) exec sh "$EMWM_PARITY_TREE/tests/run-x11.sh" ;;
    tiling) exec sh "$EMWM_PARITY_TREE/tests/run-tiling.sh" ;;
    defaults)
        export TILE_CONFIG="$EMWM_PARITY_TREE/src/system.emwmrc" TILE_BINDINGS=DefaultKeyBindings DEFAULT_TILE_BINDINGS=1
        exec sh "$EMWM_PARITY_TREE/tests/run-tiling.sh" ;;
    no-display) unset DISPLAY; exec "$EMWM_BINARY" ;;
    *) exit 2 ;;
esac
