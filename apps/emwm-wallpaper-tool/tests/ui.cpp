// Exercise the actual GTK controls on a private Xvfb server and temporary HOME.
#define main wallpaper_application_main
#include "../src/main.cpp"
#undef main

namespace {
template<class T> void collect(Gtk::Widget& widget, std::vector<T*>& matches) {
    if (auto* match = dynamic_cast<T*>(&widget)) matches.push_back(match);
    if (auto* container = dynamic_cast<Gtk::Container*>(&widget))
        for (auto* child : container->get_children()) collect(*child, matches);
}
void require(bool condition, const char* message) {
    if (!condition) throw std::runtime_error(message);
}
std::string readContents(const fs::path& path) {
    std::ifstream stream(path);
    return {std::istreambuf_iterator<char>(stream), std::istreambuf_iterator<char>()};
}
}

int main(int argc, char** argv) {
    try {
        Gtk::Main gtk(argc, argv);
        const fs::path home = getenvRequired("HOME");
        const auto resources = backdrops::resourceFile(home);
        const auto before = readContents(resources);
        MainWindow main;
        main.show();
        Gtk::Window parent;
        PlacementControls controls;
        parent.add(controls);
        parent.show_all();
        const auto source = home / "monitor-ui.png";
        wallpaper::Image image(gdk_pixbuf_new(GDK_COLORSPACE_RGB, false, 8, 4, 2));
        gdk_pixbuf_fill(image.get(), 0xff0000ff);
        require(gdk_pixbuf_save(image.get(), source.c_str(), "png", nullptr, nullptr), "save fixture");
        controls.sourcePath = [&] { return source; };
        controls.restore({source, "fit", "#123456"});
        auto edit = [&](int response, const std::string& rotation) {
            std::string failure;
            Glib::signal_timeout().connect([&] {
                for (auto* window : Gtk::Window::list_toplevels()) {
                    auto* dialog = dynamic_cast<Gtk::Dialog*>(window);
                    if (!dialog || dialog->get_title() != "Papel de parede por monitor") continue;
                    try {
                        std::vector<Gtk::FileChooserButton*> files;
                        collect(*dialog, files);
                        require(files.size() == wallpaper::detectLayout().monitors.size(), "missing monitor rows");
                        require(files.front()->get_filename() == source, "general source not offered for monitor");
                        std::vector<Gtk::ComboBoxText*> combos;
                        collect(*dialog, combos);
                        bool found = false;
                        for (auto* combo : combos) if (combo->set_active_id(rotation)) found = true;
                        require(found, "missing rotation control");
                    } catch (const std::exception& error) { failure = error.what(); }
                    dialog->response(response);
                    return false;
                }
                return true;
            }, 200);
            std::vector<Gtk::Button*> buttons;
            collect(controls, buttons);
            for (auto* button : buttons) {
                if (button->get_label().find("monitor") != Glib::ustring::npos) {
                    button->clicked();
                    if (!failure.empty()) throw std::runtime_error(failure);
                    return;
                }
            }
            throw std::runtime_error("missing monitor dialog button");
        };
        edit(Gtk::RESPONSE_CANCEL, "90");
        require(controls.selection(source).monitors.empty(), "cancel changed preferences");
        edit(Gtk::RESPONSE_OK, "90");
        auto selected = controls.selection(source);
        require(!selected.monitors.empty() && selected.monitors[0].rotation == "90" &&
                selected.monitors[0].mode == "fit" && selected.monitors[0].color == "#123456",
                "dialog did not accept independent settings");
        controls.restore(selected);
        edit(Gtk::RESPONSE_CANCEL, "180");
        require(controls.selection(source).monitors[0].rotation == "90", "cancel lost saved rotation");
        edit(Gtk::RESPONSE_REJECT, "auto");
        require(controls.selection(source).monitors.empty(), "single-image reset failed");
        require(readContents(resources) == before, "preview wrote preferences before Apply");
        std::cout << "PASS: monitor dialog, source fallback, rotation, cancel, restore and single-image reset\n";
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
