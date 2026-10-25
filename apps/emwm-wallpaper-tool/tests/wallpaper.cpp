#include "wallpaper.h"
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <unistd.h>
#include <fstream>
#include <iostream>
#include <stdexcept>

namespace fs = std::filesystem;
constexpr unsigned red = 0xff0000, blue = 0x0000ff, background = 0x123456;
void require(bool ok, const char* message) {
    if (!ok) throw std::runtime_error(message);
}
wallpaper::Image fixture(int width = 4, int height = 2) {
    wallpaper::Image image(gdk_pixbuf_new(GDK_COLORSPACE_RGB, false, 8, width, height));
    auto* pixels = gdk_pixbuf_get_pixels(image.get());
    const int stride = gdk_pixbuf_get_rowstride(image.get());
    for (int y = 0; y < height; ++y) for (int x = 0; x < width; ++x) {
        auto* p = pixels + y * stride + x * 3;
        p[0] = x < width / 2 ? 255 : 0; p[1] = 0; p[2] = x < width / 2 ? 0 : 255;
    }
    return image;
}
unsigned pixel(GdkPixbuf* image, int x, int y) {
    const auto* p = gdk_pixbuf_get_pixels(image) + y * gdk_pixbuf_get_rowstride(image) + x * 3;
    return (p[0] << 16) | (p[1] << 8) | p[2];
}
void saveImage(GdkPixbuf* image, const fs::path& path) {
    require(gdk_pixbuf_save(image, path.c_str(), "png", nullptr, nullptr), "fixture save failed");
}
void checkScreen(const std::string& mode) {
    Display* display = XOpenDisplay(nullptr);
    require(display, "no display");
    XImage* image = XGetImage(display, DefaultRootWindow(display), 0, 0, 800, 600, AllPlanes, ZPixmap);
    require(image, "no screen image");
    bool ok;
    if (mode == "monitors") {
        ok = XGetPixel(image, 100, 50) == background && XGetPixel(image, 500, 300) == background &&
             XGetPixel(image, 100, 250) == red && XGetPixel(image, 300, 250) == blue &&
             XGetPixel(image, 700, 100) == red && XGetPixel(image, 700, 500) == blue;
    } else if (mode == "fit") {
        ok = XGetPixel(image, 100, 50) == background && XGetPixel(image, 700, 550) == background &&
             XGetPixel(image, 100, 300) == red && XGetPixel(image, 700, 300) == blue;
    } else if (mode == "center") {
        ok = XGetPixel(image, 100, 50) == background && XGetPixel(image, 700, 550) == background &&
             XGetPixel(image, 385, 300) == red && XGetPixel(image, 415, 300) == blue;
    } else if (mode == "tile") {
        ok = XGetPixel(image, 100, 50) == blue && XGetPixel(image, 700, 550) == blue &&
             XGetPixel(image, 385, 300) == blue && XGetPixel(image, 415, 300) == red;
    } else {
        ok = XGetPixel(image, 100, 50) == red && XGetPixel(image, 700, 550) == blue &&
             XGetPixel(image, 100, 300) == red && XGetPixel(image, 700, 300) == blue;
    }
    if (!ok) {
        std::cerr << mode << " pixels:" << std::hex;
        for (const auto& point : {std::pair{100, 50}, std::pair{700, 550}, std::pair{100, 300},
                                  std::pair{700, 300}, std::pair{385, 300}, std::pair{415, 300}})
            std::cerr << ' ' << XGetPixel(image, point.first, point.second);
        std::cerr << std::dec << '\n';
    }
    XDestroyImage(image);
    XCloseDisplay(display);
    require(ok, "rendered wallpaper pixels differ");
}

int main(int argc, char** argv) {
    try {
        if (argc == 2 && std::string(argv[1]) == "layout") {
            const auto layout = wallpaper::detectLayout();
            require(layout.width == 800 && layout.height == 600 && !layout.monitors.empty(), "X11 detection failed");
            const auto& monitor = layout.monitors.front();
            require(monitor.width == 800 && monitor.height == 600 && monitor.rotation == 0 && !monitor.name.empty(), "RandR geometry failed");
            return 0;
        }
        if (argc == 3 && std::string(argv[1]) == "pixel") {
            checkScreen(argv[2]);
            return 0;
        }
        if (argc == 4 && std::string(argv[1]) == "fixture") {
            const fs::path home(argv[2]);
            const auto source = home / "imagem original.png";
            auto image = fixture(40, 20);
            saveImage(image.get(), source);
            const bool monitors = std::string(argv[3]) == "monitors";
            backdrops::Selection selected{source, monitors ? "fit" : argv[3], "#123456"};
            const wallpaper::Layout layout{800, 600, {{"HDMI-1", 0, 100, 400, 300, 0}, {"DP-1", 600, 0, 200, 600, 90}}};
            if (monitors) selected.monitors = {{"HDMI-1", source, "stretch", "#123456", "auto"},
                                                {"DP-1", source, "stretch", "#123456", "90"}};
            const auto output = monitors ? wallpaper::prepare(home, selected, layout) : wallpaper::prepare(home, selected, 800, 600);
            backdrops::save(home, {0}, {output}, {selected});
            std::cout << backdrops::tellmwmImageArgument(output);
            return 0;
        }
        auto source = fixture();
        for (const auto& mode : {"fit", "zoom", "stretch", "center", "tile"}) {
            auto image = wallpaper::compose(source.get(), mode, 8, 8, "#123456");
            require(gdk_pixbuf_get_width(image.get()) == 8 && gdk_pixbuf_get_height(image.get()) == 8,
                    "wrong output dimensions");
            const std::string placement(mode);
            if (placement == "fit") {
                require(pixel(image.get(), 0, 1) == background && pixel(image.get(), 7, 6) == background,
                        "fit must preserve margins");
                require(pixel(image.get(), 0, 2) == red && pixel(image.get(), 7, 5) == blue,
                        "fit placement wrong");
            } else if (placement == "center") {
                require(pixel(image.get(), 1, 3) == background && pixel(image.get(), 6, 4) == background,
                        "center must preserve native size");
                require(pixel(image.get(), 2, 3) == red && pixel(image.get(), 5, 4) == blue,
                        "center placement wrong");
            } else if (placement == "tile") {
                for (int y = 0; y < 8; ++y) for (int x = 0; x < 8; ++x)
                    require(pixel(image.get(), x, y) == (x % 4 < 2 ? red : blue), "tile pattern wrong");
            } else {
                require(pixel(image.get(), 0, 0) == red && pixel(image.get(), 7, 7) == blue,
                        "fill/stretch must cover screen");
            }
        }
        auto portrait = fixture(2, 4);
        auto fitted = wallpaper::compose(portrait.get(), "fit", 8, 4, "#123456");
        require(pixel(fitted.get(), 2, 0) == background && pixel(fitted.get(), 3, 0) == red &&
                pixel(fitted.get(), 4, 3) == blue && pixel(fitted.get(), 5, 3) == background,
                "portrait fit wrong");
        auto wide = fixture(8, 2);
        gdk_pixbuf_fill(wide.get(), 0x00ff00ff);
        wallpaper::Image middle(gdk_pixbuf_new_subpixbuf(wide.get(), 2, 0, 4, 2));
        gdk_pixbuf_fill(middle.get(), 0xff0000ff);
        auto zoomed = wallpaper::compose(wide.get(), "zoom", 8, 8, "#123456");
        auto stretched = wallpaper::compose(wide.get(), "stretch", 8, 8, "#123456");
        require(pixel(zoomed.get(), 0, 0) == red && pixel(zoomed.get(), 7, 7) == red,
                "zoom must crop both sides while preserving aspect ratio");
        require(pixel(stretched.get(), 0, 0) == 0x00ff00 && pixel(stretched.get(), 7, 7) == 0x00ff00,
                "stretch must retain the edges");
        auto cropped = wallpaper::compose(source.get(), "center", 2, 2, "#123456");
        require(pixel(cropped.get(), 0, 0) == red && pixel(cropped.get(), 1, 1) == blue,
                "center must crop large images");
        wallpaper::Image transparent(gdk_pixbuf_new(GDK_COLORSPACE_RGB, true, 8, 1, 1));
        gdk_pixbuf_fill(transparent.get(), 0xff000000);
        for (const auto& mode : {"fit", "zoom", "stretch", "center", "tile"}) {
            auto image = wallpaper::compose(transparent.get(), mode, 7, 5, "#123456");
            for (int y = 0; y < 5; ++y) for (int x = 0; x < 7; ++x)
                require(pixel(image.get(), x, y) == background, "alpha must use background color");
        }
        for (const auto& mode : {"invalid", "fit"}) {
            bool rejected = false;
            try { wallpaper::compose(source.get(), mode, std::string(mode) == "fit" ? 0 : 8, 8, "#123456"); }
            catch (const std::exception&) { rejected = true; }
            require(rejected, "invalid input accepted");
        }
        char pattern[] = "/tmp/emwm-render-test-XXXXXX";
        const char* temp = mkdtemp(pattern);
        require(temp, "mkdtemp failed");
        const fs::path home(temp);
        struct Cleanup { fs::path path; ~Cleanup() { fs::remove_all(path); } } cleanup{home};
        const auto original = home / "original com espaço.png";
        saveImage(source.get(), original);
        backdrops::Selection selection{original, "fit", "#123456"};
        const auto output = wallpaper::prepare(home, selection, 8, 8);
        require(output == wallpaper::prepare(home, selection, 8, 8), "identical image not reused");
        require(fs::exists(original) && output != original, "original overwritten");
        backdrops::save(home, {0}, {output}, {selection});
        const auto saved = backdrops::readSelections(home)[0];
        require(saved && saved->source == original && saved->mode == "fit" && saved->color == "#123456",
                "preferences not restored");
        backdrops::save(home, {1}, {original});
        backdrops::save(home, {}, {});
        require(backdrops::readSelections(home)[0]->mode == "fit" && backdrops::readSaved(home)[0] == output,
                "partial update/migration lost rendered selection");
        require(backdrops::readSelections(home)[1]->mode == "tile", "legacy default changed");
        selection.mode = "zoom";
        require(output != wallpaper::prepare(home, selection, 8, 8), "changed render reused stale filename");
        gdk_pixbuf_fill(source.get(), 0x00ff00ff);
        saveImage(source.get(), original);
        selection.mode = "fit";
        require(output != wallpaper::prepare(home, selection, 8, 8), "changed source reused stale filename");
        // Offset landscape + portrait, with a gap and different source pictures.
        auto second = fixture();
        const auto other = home / "segunda imagem.png";
        saveImage(second.get(), other);
        const wallpaper::Layout layout{12, 8, {{"HDMI-1", 0, 2, 6, 4, 0}, {"DP-1", 8, 0, 4, 8, 90}}};
        selection = {{}, "fit", "#123456", {{"HDMI-1", original, "stretch", "#000000", "auto"},
                                           {"DP-1", other, "stretch", "#ffffff", "auto"}}};
        auto desktop = wallpaper::composeDesktop(selection, layout);
        require(pixel(desktop.get(), 0, 0) == background && pixel(desktop.get(), 7, 4) == background,
                "monitor offsets/gaps lost");
        require(pixel(desktop.get(), 0, 2) == 0x00ff00 && pixel(desktop.get(), 5, 5) == 0x00ff00,
                "landscape source not used");
        require(pixel(desktop.get(), 8, 0) == red && pixel(desktop.get(), 11, 7) == blue,
                "portrait auto must not double rotate");
        for (const auto& rotation : {"0", "90", "180", "270"}) {
            selection.monitors[1].rotation = rotation;
            desktop = wallpaper::composeDesktop(selection, layout);
            const bool reverse = std::string(rotation) == "180" || std::string(rotation) == "270";
            require(pixel(desktop.get(), 8, 0) == (reverse ? blue : red) &&
                    pixel(desktop.get(), 11, 7) == (reverse ? red : blue), "rotation corners wrong");
            if (std::string(rotation) == "90" || std::string(rotation) == "270")
                require(pixel(desktop.get(), 11, 0) == (reverse ? blue : red), "rotation must swap axes");
        }
        selection.monitors.push_back({"Desconectado", other, "tile", "#112233", "180"});
        const auto composite = wallpaper::prepare(home, selection, layout);
        backdrops::save(home, {2}, {composite}, {selection});
        backdrops::save(home, {3}, {original});
        backdrops::save(home, {}, {});
        auto restored = *backdrops::readSelections(home)[2];
        require(restored.source.empty() && restored.monitors.size() == 3 &&
                restored.monitors[1].name == "DP-1" && restored.monitors[1].source == other &&
                restored.monitors[1].rotation == "270" && restored.monitors[2].color == "#112233",
                "monitor settings lost on save/migration/partial update");
        require(wallpaper::prepare(home, restored, layout) == composite, "monitor roundtrip changes image");
        auto disconnected = layout;
        disconnected.monitors.erase(disconnected.monitors.begin()+1);
        desktop = wallpaper::composeDesktop(restored, disconnected);
        require(pixel(desktop.get(), 8, 0) == background, "disconnected monitor still rendered");
        auto added = layout;
        added.monitors[1].name = "New Output";
        bool rejected = false;
        try { wallpaper::composeDesktop(restored, added); } catch (const std::exception&) { rejected = true; }
        require(rejected, "new monitor without source silently accepted");
        restored.source = original;
        desktop = wallpaper::composeDesktop(restored, added);
        require(pixel(desktop.get(), 9, 3) == 0x00ff00, "new monitor fallback missing");
        restored.monitors[0].rotation = "invalid";
        rejected = false;
        try { backdrops::save(home, {2}, {composite}, {restored}); } catch (const std::exception&) { rejected = true; }
        require(rejected && backdrops::readSelections(home)[2]->monitors[0].rotation == "auto",
                "invalid rotation changed persisted settings");
        std::cout << "PASS: multi-monitor composition, rotations, offsets, hotplug and persistence\n";
        std::cout << "PASS: placement, margins, cropping, alpha, prepared images and saved preferences\n";
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
