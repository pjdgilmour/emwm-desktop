#include "wallpaper.h"

#include <X11/Xlib.h>
#include <X11/extensions/Xrandr.h>
#include <algorithm>
#include <cmath>
#include <stdexcept>

namespace fs = std::filesystem;
namespace wallpaper {
namespace {
void dimensions(int width, int height) {
    if (width <= 0 || height <= 0 || width > 32768 || height > 32768 ||
        static_cast<long long>(width) * height > 100000000)
        throw std::runtime_error("Resolução ou imagem inválida (máximo de 100 megapixels).");
}

std::runtime_error failure(const std::string& message, GError* error) {
    const auto detail = error ? std::string(error->message) : "erro desconhecido";
    if (error) g_error_free(error);
    return std::runtime_error(message + ": " + detail);
}
}

Image compose(GdkPixbuf* source, const std::string& mode, int width, int height,
              const std::string& color) {
    dimensions(width, height);
    if (!source || !backdrops::validMode(mode) || !backdrops::validColor(color))
        throw std::runtime_error("Imagem, posicionamento ou cor de fundo inválidos.");
    const int sw = gdk_pixbuf_get_width(source), sh = gdk_pixbuf_get_height(source);
    dimensions(sw, sh);
    Image canvas(gdk_pixbuf_new(GDK_COLORSPACE_RGB, false, 8, width, height));
    if (!canvas) throw std::runtime_error("Não foi possível criar a imagem do fundo.");
    gdk_pixbuf_fill(canvas.get(), (std::stoul(color.substr(1), nullptr, 16) << 8) | 0xff);

    if (mode == "tile") {
        int filledWidth = std::min(sw, width), filledHeight = std::min(sh, height);
        gdk_pixbuf_composite(source, canvas.get(), 0, 0, filledWidth, filledHeight,
                             0, 0, 1, 1, GDK_INTERP_NEAREST, 255);
        // Double filled regions so even a 1x1 pattern is inexpensive.
        while (filledWidth < width) {
            const int amount = std::min(filledWidth, width - filledWidth);
            gdk_pixbuf_copy_area(canvas.get(), 0, 0, amount, filledHeight,
                                 canvas.get(), filledWidth, 0);
            filledWidth += amount;
        }
        while (filledHeight < height) {
            const int amount = std::min(filledHeight, height - filledHeight);
            gdk_pixbuf_copy_area(canvas.get(), 0, 0, width, amount,
                                 canvas.get(), 0, filledHeight);
            filledHeight += amount;
        }
        return canvas;
    }

    double sx = 1, sy = 1;
    int dw = sw, dh = sh;
    if (mode == "stretch") {
        dw = width; dh = height;
        sx = double(width) / sw; sy = double(height) / sh;
    } else if (mode == "fit" || mode == "zoom") {
        const double scale = mode == "fit" ? std::min(double(width) / sw, double(height) / sh)
                                           : std::max(double(width) / sw, double(height) / sh);
        dw = std::max(1, int(std::round(sw * scale)));
        dh = std::max(1, int(std::round(sh * scale)));
        sx = double(dw) / sw; sy = double(dh) / sh;
    }
    const int x = (width - dw) / 2, y = (height - dh) / 2;
    gdk_pixbuf_composite(source, canvas.get(), std::max(0, x), std::max(0, y),
                         std::min(width, dw), std::min(height, dh), x, y, sx, sy,
                         mode == "center" ? GDK_INTERP_NEAREST : GDK_INTERP_BILINEAR, 255);
    return canvas;
}

Layout detectLayout() {
    std::unique_ptr<Display, decltype(&XCloseDisplay)> display(XOpenDisplay(nullptr), XCloseDisplay);
    if (!display) throw std::runtime_error("Não foi possível consultar os monitores X11.");
    const int screen = DefaultScreen(display.get());
    Layout layout{DisplayWidth(display.get(), screen), DisplayHeight(display.get(), screen), {}};
    int event, error;
    if (XRRQueryExtension(display.get(), &event, &error)) {
        std::unique_ptr<XRRScreenResources, decltype(&XRRFreeScreenResources)> resources(
            XRRGetScreenResourcesCurrent(display.get(), RootWindow(display.get(), screen)), XRRFreeScreenResources);
        if (resources) for (int i = 0; i < resources->noutput; ++i) {
            std::unique_ptr<XRROutputInfo, decltype(&XRRFreeOutputInfo)> output(
                XRRGetOutputInfo(display.get(), resources.get(), resources->outputs[i]), XRRFreeOutputInfo);
            if (!output || output->connection != RR_Connected || !output->crtc) continue;
            std::unique_ptr<XRRCrtcInfo, decltype(&XRRFreeCrtcInfo)> crtc(
                XRRGetCrtcInfo(display.get(), resources.get(), output->crtc), XRRFreeCrtcInfo);
            if (!crtc || !crtc->width || !crtc->height) continue;
            // Mirrored outputs share pixels and must be configured together.
            const auto duplicate = std::find_if(layout.monitors.begin(), layout.monitors.end(), [&](const Monitor& m) {
                return m.x == crtc->x && m.y == crtc->y && m.width == int(crtc->width) && m.height == int(crtc->height);
            });
            if (duplicate != layout.monitors.end()) continue;
            const int rotation = crtc->rotation & RR_Rotate_90 ? 90 :
                                 crtc->rotation & RR_Rotate_180 ? 180 :
                                 crtc->rotation & RR_Rotate_270 ? 270 : 0;
            layout.monitors.push_back({std::string(output->name, output->nameLen), crtc->x, crtc->y,
                                       int(crtc->width), int(crtc->height), rotation});
        }
    }
    if (layout.monitors.empty()) layout.monitors.push_back({"Tela X11", 0, 0, layout.width, layout.height, 0});
    return layout;
}

Image load(const fs::path& path) {
    int sw = 0, sh = 0;
    if (path.empty() || !gdk_pixbuf_get_file_info(path.c_str(), &sw, &sh))
        throw std::runtime_error("Não foi possível ler a imagem: " + path.string());
    dimensions(sw, sh);
    GError* error = nullptr;
    Image source(gdk_pixbuf_new_from_file(path.c_str(), &error));
    if (!source) throw failure("Falha ao abrir a imagem", error);
    Image oriented(gdk_pixbuf_apply_embedded_orientation(source.get()));
    if (!oriented) throw std::runtime_error("Não foi possível orientar a imagem.");
    return oriented;
}

Image composeDesktop(const backdrops::Selection& selection, const Layout& layout) {
    dimensions(layout.width, layout.height);
    if (selection.monitors.empty()) {
        auto source = load(selection.source);
        return compose(source.get(), selection.mode, layout.width, layout.height, selection.color);
    }
    if (!backdrops::validColor(selection.color) || layout.monitors.empty())
        throw std::runtime_error("Configuração de tela inválida.");
    Image canvas(gdk_pixbuf_new(GDK_COLORSPACE_RGB, false, 8, layout.width, layout.height));
    if (!canvas) throw std::runtime_error("Não foi possível criar a imagem do fundo.");
    gdk_pixbuf_fill(canvas.get(), (std::stoul(selection.color.substr(1), nullptr, 16) << 8) | 0xff);
    for (const auto& monitor : layout.monitors) {
        dimensions(monitor.width, monitor.height);
        if (monitor.x < 0 || monitor.y < 0 || monitor.x > layout.width - monitor.width ||
            monitor.y > layout.height - monitor.height)
            throw std::runtime_error("Monitor fora da área da tela: " + monitor.name);
        auto found = std::find_if(selection.monitors.begin(), selection.monitors.end(),
                                  [&](const auto& m) { return m.name == monitor.name; });
        backdrops::MonitorSelection selected{monitor.name, selection.source, selection.mode, selection.color, "auto"};
        if (found != selection.monitors.end()) selected = *found;
        if (selected.source.empty()) throw std::runtime_error("Selecione uma imagem para o monitor " + monitor.name + ".");
        if (!backdrops::validRotation(selected.rotation)) throw std::runtime_error("Rotação inválida.");
        auto source = load(selected.source);
        // RandR geometry is already rotated. Auto must not rotate the picture twice.
        // Manual angles rotate the picture clockwise within that logical rectangle.
        if (selected.rotation != "auto" && selected.rotation != "0") {
            source.reset(gdk_pixbuf_rotate_simple(source.get(),
                selected.rotation == "90" ? GDK_PIXBUF_ROTATE_CLOCKWISE :
                selected.rotation == "180" ? GDK_PIXBUF_ROTATE_UPSIDEDOWN : GDK_PIXBUF_ROTATE_COUNTERCLOCKWISE));
            if (!source) throw std::runtime_error("Não foi possível girar a imagem.");
        }
        auto tile = compose(source.get(), selected.mode, monitor.width, monitor.height, selected.color);
        gdk_pixbuf_copy_area(tile.get(), 0, 0, monitor.width, monitor.height, canvas.get(), monitor.x, monitor.y);
    }
    return canvas;
}

fs::path prepare(const fs::path& home, const backdrops::Selection& selection,
                 int width, int height) {
    return prepare(home, selection, Layout{width, height, {{"Tela X11", 0, 0, width, height, 0}}});
}

fs::path prepare(const fs::path& home, const backdrops::Selection& selection, const Layout& layout) {
    auto canvas = composeDesktop(selection, layout);
    GError* error = nullptr;
    // Motif 2.3.8 can lose the red/green channels of RGB PNGs. An opaque
    // RGBA PNG preserves the exact colors in its native image loader.
    Image encoded(gdk_pixbuf_add_alpha(canvas.get(), false, 0, 0, 0));
    if (!encoded) throw std::runtime_error("Não foi possível preparar a imagem para o EMWM.");
    gchar* buffer = nullptr;
    gsize size = 0;
    if (!gdk_pixbuf_save_to_buffer(encoded.get(), &buffer, &size, "png", &error, nullptr))
        throw failure("Falha ao preparar o fundo", error);
    std::unique_ptr<gchar, decltype(&g_free)> bytes(buffer, g_free);
    std::unique_ptr<gchar, decltype(&g_free)> hash(
        g_compute_checksum_for_data(G_CHECKSUM_SHA256, reinterpret_cast<guchar*>(buffer), size), g_free);
    const auto directory = backdrops::resourceFile(home).parent_path() / "wallpapers";
    fs::create_directories(directory);
    // Content-based names prevent stale pixmaps in Motif's filename cache.
    const auto output = directory / (std::string(hash.get()) + ".png");
    if (!g_file_set_contents_full(output.c_str(), buffer, size,
                                  G_FILE_SET_CONTENTS_CONSISTENT, 0600, &error))
        throw failure("Falha ao salvar o fundo preparado", error);
    return output;
}
}
