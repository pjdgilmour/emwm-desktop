#include <gtkmm.h>

#include <algorithm>
#include <cctype>
#include <cmath>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace fs = std::filesystem;

namespace {

struct ColorOption {
    std::string field;
    std::string label;
    std::string color;
    Gtk::EventBox* square = nullptr;
};

std::string trim(const std::string& value) {
    const auto first = std::find_if_not(value.begin(), value.end(), [](unsigned char c) {
        return std::isspace(c);
    });
    const auto last = std::find_if_not(value.rbegin(), value.rend(), [](unsigned char c) {
        return std::isspace(c);
    }).base();

    if (first >= last) {
        return "";
    }

    return std::string(first, last);
}

fs::path xresourcesPath() {
    const char* home = std::getenv("HOME");
    if (!home || std::string(home).empty()) {
        throw std::runtime_error("Não foi possível encontrar a variável HOME.");
    }

    return fs::path(home) / ".Xresources";
}

Gdk::RGBA colorFromString(const std::string& color) {
    Gdk::RGBA rgba;
    if (!color.empty() && rgba.set(color)) {
        return rgba;
    }

    rgba.set_rgba(1.0, 1.0, 1.0, 1.0);
    return rgba;
}

std::string colorToHex(const Gdk::RGBA& color) {
    const auto channel = [](double value) {
        return std::clamp(static_cast<int>(std::lround(value * 255.0)), 0, 255);
    };

    std::ostringstream hex;
    hex << '#'
        << std::hex << std::nouppercase << std::setfill('0')
        << std::setw(2) << channel(color.get_red())
        << std::setw(2) << channel(color.get_green())
        << std::setw(2) << channel(color.get_blue());
    return hex.str();
}

class ThemeWindow : public Gtk::Window {
public:
    ThemeWindow()
        : options_({
              {"Emwm*client*background", "Decoração da janela inativa", ""},
              {"Emwm*client*activeBackground", "Decoração da janela ativa", ""},
              {"Emwm*icon*activeBackground", "Cor de seleção do ícone", ""},
          }) {
        set_title("Ajustar o tema");
        set_border_width(10);
        set_resizable(false);

        grid_.set_row_spacing(8);
        grid_.set_column_spacing(10);
        add(grid_);

        loadColors();
        buildUi();
        show_all_children();
    }

private:
    void showError(const Glib::ustring& message) {
        Gtk::MessageDialog dialog(*this, message, false, Gtk::MESSAGE_ERROR, Gtk::BUTTONS_OK, true);
        dialog.run();
    }

    void loadColors() {
        std::ifstream file(xresourcesPath());
        if (!file.good()) {
            return;
        }

        std::string line;
        while (std::getline(file, line)) {
            for (auto& option : options_) {
                if (line.find(option.field) == std::string::npos) {
                    continue;
                }

                const auto separator = line.find(':');
                if (separator != std::string::npos) {
                    option.color = trim(line.substr(separator + 1));
                }
            }
        }
    }

    void buildUi() {
        for (std::size_t i = 0; i < options_.size(); ++i) {
            auto& option = options_[i];

            auto* label = Gtk::make_managed<Gtk::Label>(option.label);
            label->set_halign(Gtk::ALIGN_START);
            grid_.attach(*label, 0, static_cast<int>(i), 1, 1);

            auto* square = Gtk::make_managed<Gtk::EventBox>();
            square->set_size_request(58, 22);
            square->override_background_color(colorFromString(option.color), Gtk::STATE_FLAG_NORMAL);
            option.square = square;
            grid_.attach(*square, 1, static_cast<int>(i), 1, 1);

            auto* button = Gtk::make_managed<Gtk::Button>("Selecionar");
            button->signal_clicked().connect([this, &option]() {
                chooseColor(option);
            });
            grid_.attach(*button, 2, static_cast<int>(i), 1, 1);
        }

        auto* save_button = Gtk::make_managed<Gtk::Button>("Salvar");
        save_button->signal_clicked().connect([this]() {
            saveColors();
        });
        grid_.attach(*save_button, 0, static_cast<int>(options_.size()), 3, 1);
    }

    void chooseColor(ColorOption& option) {
        Gtk::ColorChooserDialog dialog("Selecionar cor", *this);
        dialog.set_rgba(colorFromString(option.color));

        if (dialog.run() != Gtk::RESPONSE_OK) {
            return;
        }

        const Gdk::RGBA color = dialog.get_rgba();
        option.color = colorToHex(color);
        if (option.square) {
            option.square->override_background_color(color, Gtk::STATE_FLAG_NORMAL);
        }
    }

    void saveColors() {
        try {
            const fs::path path = xresourcesPath();
            std::ifstream input(path);
            if (!input.good()) {
                showError("Não foi possível abrir ~/.Xresources para leitura.");
                return;
            }

            std::vector<std::string> lines;
            std::string line;
            while (std::getline(input, line)) {
                for (const auto& option : options_) {
                    if (line.find(option.field) != std::string::npos) {
                        line = option.field + ": " + option.color;
                    }
                }
                lines.push_back(line);
            }
            input.close();

            std::ofstream output(path, std::ios::trunc);
            if (!output.good()) {
                showError("Não foi possível abrir ~/.Xresources para escrita.");
                return;
            }

            for (const auto& saved_line : lines) {
                output << saved_line << '\n';
            }

            Gtk::MessageDialog dialog(*this, "As cores foram salvas com sucesso!", false, Gtk::MESSAGE_INFO, Gtk::BUTTONS_OK, true);
            dialog.set_title("Sucesso");
            dialog.run();
            hide();
        } catch (const std::exception& error) {
            showError(error.what());
        }
    }

    Gtk::Grid grid_;
    std::vector<ColorOption> options_;
};

}  // namespace

int main(int argc, char* argv[]) {
    auto app = Gtk::Application::create(argc, argv, "br.local.trocacor");

    ThemeWindow window;
    return app->run(window);
}
