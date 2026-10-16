#include "backdrops.h"
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <unistd.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>

namespace fs = std::filesystem;
void require(bool ok, const char* message) {
    if (!ok) throw std::runtime_error(message);
}
void write(const fs::path& path, const std::string& data) {
    fs::create_directories(path.parent_path());
    std::ofstream output(path);
    output << data;
    require(output.good(), "write failed");
}
std::string read(const fs::path& path) {
    std::ifstream input(path);
    return {std::istreambuf_iterator<char>(input), std::istreambuf_iterator<char>()};
}
std::string shellQuote(const std::string& value) {
    std::string result = "'";
    for (char c : value) result += c == '\'' ? "'\\''" : std::string(1, c);
    return result + '\'';
}
void makeFixture(const fs::path& home) {
    const auto image = home / "fundo com espaço 'aspas' \"duplas\" \\ barra #1.xpm";
    write(image, "/* XPM */\nstatic char *image[] = {\n\"2 2 1 1\",\n\"a c #d02040\",\n\"aa\",\n\"aa\"};\n");
    write(home / "image-path", image.string());
    write(home / ".config/emwm/apply-backdrops.sh", "#!/bin/sh\nsleep 3\n/usr/bin/tellmwm backdrop ws0 " + shellQuote(image.string()) + "\n");
    write(home / ".Xresources", "! configuração anterior\nXTerm*faceSize: 13\nEmwm*workspaceCount: 4\n");
}
int main(int argc, char** argv) {
    try {
        if (argc == 3 && std::string(argv[1]) == "fixture") {
            makeFixture(argv[2]);
            return 0;
        }
        if (argc == 3 && std::string(argv[1]) == "argument") {
            std::cout << backdrops::tellmwmImageArgument(argv[2]);
            return 0;
        }
        if (argc == 2 && std::string(argv[1]) == "pixel") {
            Display* d = XOpenDisplay(nullptr);
            require(d, "no display");
            XImage* image = XGetImage(d, DefaultRootWindow(d), 400, 300, 1, 1, AllPlanes, ZPixmap);
            require(image, "no image");
            const auto pixel = XGetPixel(image, 0, 0);
            XDestroyImage(image);
            XCloseDisplay(d);
            require(pixel == 0xd02040, "wallpaper pixel differs from saved image");
            std::cout << "PASS: native backdrop pixel d02040\n";
            return 0;
        }
        char pattern[] = "/tmp/emwm-wallpaper-test-XXXXXX";
        const char* dir = ::mkdtemp(pattern);
        require(dir, "mkdtemp failed");
        const fs::path home(dir);
        struct Cleanup { fs::path dir; ~Cleanup() { fs::remove_all(dir); } } cleanup{home};
        makeFixture(home);
        const std::string original = read(home / ".Xresources");
        const auto expected = fs::path(read(home / "image-path"));
        require(backdrops::readSaved(home)[0] == expected, "legacy quoting lost");
        backdrops::save(home, {}, {});
        require(backdrops::readSaved(home)[0] == expected, "Xrm escaping lost");
        const auto config = read(home / ".Xresources");
        require(config.find(original) == 0, "existing config changed");
        require(read(home / ".Xresources.before-wallpaper-resources") == original, "backup differs");
        backdrops::save(home, {1}, {home / "outro fundo.png"});
        auto paths = backdrops::readSaved(home);
        require(paths[0] == expected && paths[1] == home / "outro fundo.png", "partial update lost a workspace");
        require(read(home / ".Xresources") == config, "duplicated include");
        require(!paths[2] && !paths[3], "invented workspace");
        // Once migrated, stale legacy choices must not override resource values.
        write(home / ".config/emwm/apply-backdrops.sh", "tellmwm backdrop ws0 '/wrong.png'\n");
        require(backdrops::readSaved(home)[0] == expected, "legacy overrode new settings");
        const auto saved = read(backdrops::resourceFile(home));
        for (const auto& bad : {std::string("bad\nname.png"), std::string("bad.png ")}) {
            bool rejected = false;
            try { backdrops::save(home, {0}, {home / bad}); } catch (...) { rejected = true; }
            require(rejected, "unrepresentable path accepted");
            require(read(backdrops::resourceFile(home)) == saved, "failed save corrupted config");
        }
        bool rejected = false;
        try { backdrops::save(home, {4}, {expected}); } catch (...) { rejected = true; }
        require(rejected, "invalid workspace accepted");
        // Preserve dotfile symlinks and target contents.
        fs::rename(home / ".Xresources", home / "dotfile");
        fs::create_symlink("dotfile", home / ".Xresources");
        write(home / "dotfile", original);
        backdrops::save(home, {}, {});
        require(fs::is_symlink(home / ".Xresources"), "dotfile symlink replaced");
        require(read(home / "dotfile") == config, "symlink target not updated");
        std::cout << "PASS: migration, quoting, partial updates, backups, idempotence, invalid paths and symlinks\n";
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
