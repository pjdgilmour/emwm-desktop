#!/bin/sh
# Install the appearance tool/helper independently of the full desktop package.
set -eu
if [ "$(id -u)" -ne 0 ]; then
    echo 'Execute: sudo ./apps/emwm-appearance/install-system.sh' >&2
    exit 1
fi
source_dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
/usr/bin/apt-get install python3 python3-gi python3-pil gir1.2-gtk-3.0 xsettingsd pkexec x11-xserver-utils
install -d -m 755 /usr/lib/emwm-desktop/appearance
for file in core.py gui.py runtime.py login_helper.py wallpaper.py wallpaper_ui.py; do
    install -o root -g root -m 644 "$source_dir/$file" "/usr/lib/emwm-desktop/appearance/$file"
done
install -o root -g root -m 755 "$source_dir/emwm-appearance" /usr/bin/emwm-appearance
install -o root -g root -m 644 "$source_dir/emwm-appearance.desktop" /usr/share/applications/emwm-appearance.desktop
printf '%s\n' 'Instalado. Abra Aparência do EMWM no menu de aplicativos ou execute emwm-appearance.'
