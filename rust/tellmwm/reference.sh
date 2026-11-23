#!/bin/sh
# The harness supplies its temporary build of third_party/tellmwm.
exec "${EMWM_TELLMWM_REFERENCE:?set the path to the original C binary}" "$@"
