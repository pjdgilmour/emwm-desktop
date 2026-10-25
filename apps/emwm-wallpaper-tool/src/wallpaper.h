#pragma once

#include "backdrops.h"
#include <gdk-pixbuf/gdk-pixbuf.h>
#include <memory>

namespace wallpaper {
struct Unref { void operator()(GdkPixbuf* value) const { g_object_unref(value); } };
using Image = std::unique_ptr<GdkPixbuf, Unref>;
struct Monitor {
    std::string name;
    int x, y, width, height;
    int rotation = 0;
};
struct Layout {
    int width, height;
    std::vector<Monitor> monitors;
};
Layout detectLayout();
Image composeDesktop(const backdrops::Selection& selection, const Layout& layout);

// Compose in physical pixels over an opaque background, including alpha images.
Image compose(GdkPixbuf* source, const std::string& mode, int width, int height,
              const std::string& color);
std::filesystem::path prepare(const std::filesystem::path& home,
                              const backdrops::Selection& selection, int width, int height);
std::filesystem::path prepare(const std::filesystem::path& home,
                              const backdrops::Selection& selection, const Layout& layout);
}
