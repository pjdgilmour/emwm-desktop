#include <gtkmm.h>
#include "backdrops.h"
#include "wallpaper.h"
#include <X11/Xlib.h>
#include <map>
#include <functional>
#include <tuple>
#include <iostream>

#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include <array>
#include <algorithm>
#include <cerrno>
#include <chrono>
#include <cctype>
#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <ctime>
#include <iomanip>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace fs = std::filesystem;

namespace {

constexpr int kWorkspaceCount = backdrops::workspaceCount;

struct CommandResult {
    int exit_code = -1;
    std::string output;
    std::string error;
};

struct ApplyPlan {
    std::vector<int> workspaces;
    std::vector<backdrops::Selection> selections;
    std::vector<fs::path> outputs;
};

struct TellmwmResult {
    bool command_found = false;
    bool all_applied = false;
    std::string details;
};

std::string getenvRequired(const char* name) {
    const char* value = std::getenv(name);
    if (!value || std::string(value).empty()) {
        throw std::runtime_error(std::string("Variável de ambiente não encontrada: ") + name);
    }
    return value;
}

std::vector<std::string> splitPath(const std::string& path_env) {
    std::vector<std::string> items;
    std::stringstream ss(path_env);
    std::string item;
    while (std::getline(ss, item, ':')) {
        if (!item.empty()) {
            items.push_back(item);
        }
    }
    return items;
}

bool commandExists(const std::string& command) {
    const char* path_env = std::getenv("PATH");
    if (!path_env) {
        return false;
    }

    for (const auto& dir : splitPath(path_env)) {
        fs::path candidate = fs::path(dir) / command;
        if (::access(candidate.c_str(), X_OK) == 0) {
            return true;
        }
    }

    return false;
}

std::string shellLikeCommand(const std::vector<std::string>& args) {
    std::ostringstream command;
    for (size_t i = 0; i < args.size(); ++i) {
        if (i > 0) {
            command << ' ';
        }
        command << '\'';
        for (char c : args[i]) {
            if (c == '\'') {
                command << "'\\''";
            } else {
                command << c;
            }
        }
        command << '\'';
    }
    return command.str();
}

void appendDebugLog(const std::string& message) {
    const char* home = std::getenv("HOME");
    if (!home || std::string(home).empty()) {
        return;
    }

    std::ofstream log(fs::path(home) / ".emwm-wallpaper-tool.log", std::ios::app);
    if (!log.good()) {
        return;
    }

    const auto now = std::chrono::system_clock::now();
    const std::time_t time = std::chrono::system_clock::to_time_t(now);
    std::tm local_time{};
    localtime_r(&time, &local_time);

    log << std::put_time(&local_time, "%Y-%m-%d %H:%M:%S") << " " << message << '\n';
}

CommandResult runCommand(const std::vector<std::string>& args, bool capture_output = false) {
    if (args.empty()) {
        throw std::runtime_error("Comando vazio.");
    }

    int output_pipe[2] = {-1, -1};

    if (capture_output) {
        if (::pipe(output_pipe) != 0) {
            throw std::runtime_error("Falha ao criar pipe para executar comando.");
        }
    }

    pid_t pid = ::fork();
    if (pid < 0) {
        throw std::runtime_error("Falha ao iniciar processo externo.");
    }

    if (pid == 0) {
        if (capture_output) {
            ::close(output_pipe[0]);
            ::dup2(output_pipe[1], STDOUT_FILENO);
            ::dup2(output_pipe[1], STDERR_FILENO);
            ::close(output_pipe[1]);
        }

        std::vector<char*> argv;
        argv.reserve(args.size() + 1);
        for (const auto& arg : args) {
            argv.push_back(const_cast<char*>(arg.c_str()));
        }
        argv.push_back(nullptr);

        ::execvp(argv[0], argv.data());
        _exit(127);
    }

    if (capture_output) {
        ::close(output_pipe[1]);
    }

    CommandResult result;

    if (capture_output) {
        std::array<char, 4096> buffer{};
        ssize_t bytes = 0;
        while ((bytes = ::read(output_pipe[0], buffer.data(), buffer.size())) > 0) {
            result.output.append(buffer.data(), static_cast<size_t>(bytes));
        }
        ::close(output_pipe[0]);
    }

    int status = 0;
    if (::waitpid(pid, &status, 0) < 0) {
        throw std::runtime_error("Falha ao aguardar processo externo.");
    }

    if (WIFEXITED(status)) {
        result.exit_code = WEXITSTATUS(status);
    } else if (WIFSIGNALED(status)) {
        result.exit_code = 128 + WTERMSIG(status);
    }

    return result;
}

std::string resolutionText(const std::pair<int, int>& resolution) {
    return std::to_string(resolution.first) + "x" + std::to_string(resolution.second);
}

std::string toLower(std::string value) {
    std::transform(value.begin(), value.end(), value.begin(), [](unsigned char c) {
        return static_cast<char>(std::tolower(c));
    });
    return value;
}

bool hasAcceptedImageExtension(const fs::path& input) {
    const std::string extension = toLower(input.extension().string());
    return extension == ".jpg" ||
           extension == ".jpeg" ||
           extension == ".png" ||
           extension == ".bmp" ||
           extension == ".webp" ||
           extension == ".tiff" ||
           extension == ".tif" ||
           extension == ".xpm";
}

void ensureReadableImage(const fs::path& input) {
    if (input.empty()) {
        throw std::runtime_error("Nenhuma imagem foi selecionada.");
    }
    if (!hasAcceptedImageExtension(input)) {
        throw std::runtime_error("Formato de imagem não aceito. Use: .jpg, .jpeg, .png, .bmp, .webp, .tiff, .tif ou .xpm");
    }
    if (!fs::exists(input) || !fs::is_regular_file(input)) {
        throw std::runtime_error("A imagem selecionada não existe: " + input.string());
    }
    std::ifstream stream(input, std::ios::binary);
    if (!stream.good()) {
        throw std::runtime_error("Não foi possível ler a imagem: " + input.string());
    }
}

std::string mergeBackdropResources(const fs::path& resources) {
    if (!commandExists("xrdb"))
        return "xrdb não encontrado. Os fundos foram salvos, mas os recursos da sessão não foram atualizados.";
    const auto result = runCommand({"xrdb", "-merge", "-nocpp", resources.string()}, true);
    if (result.exit_code != 0)
        return "Os fundos foram salvos, mas não foi possível atualizar os recursos da sessão:\n" + result.output;
    return {};
}

TellmwmResult applyWithTellmwm(const std::vector<int>& workspaces, const std::vector<fs::path>& outputs) {
    TellmwmResult result;

    if (!commandExists("tellmwm")) {
        result.details = "tellmwm não encontrado; o fundo será aplicado no próximo login do EMWM.";
        return result;
    }

    result.command_found = true;
    result.all_applied = true;

    for (size_t i = 0; i < workspaces.size(); ++i) {
        const std::string workspace_id = "ws" + std::to_string(workspaces[i]);
        const std::vector<std::string> args = {
            "tellmwm",
            "backdrop",
            workspace_id,
            backdrops::tellmwmImageArgument(outputs[i])
        };

        appendDebugLog("Executando: " + shellLikeCommand(args));
        CommandResult backdrop_result = runCommand(args, true);
        appendDebugLog(
            "Resultado tellmwm " + workspace_id +
            ": exit=" + std::to_string(backdrop_result.exit_code) +
            " output=[" + backdrop_result.output + "]"
        );

        if (backdrop_result.exit_code != 0) {
            result.all_applied = false;
            std::string details = backdrop_result.error.empty() ? backdrop_result.output : backdrop_result.error;
            if (!result.details.empty()) {
                result.details += "\n";
            }
            result.details += "Falha ao aplicar " + workspace_id + " com tellmwm.";
            if (!details.empty()) {
                result.details += "\n" + details;
            }
        }
    }

    if (result.all_applied) {
        result.details = "O fundo foi aplicado imediatamente com tellmwm.";
    }

    return result;
}

class PlacementControls : public Gtk::Box {
public:
    explicit PlacementControls(bool monitor_tools = true) : Gtk::Box(Gtk::ORIENTATION_VERTICAL, 6) {
        auto* row = Gtk::make_managed<Gtk::Box>(Gtk::ORIENTATION_HORIZONTAL, 8);
        pack_start(*row, Gtk::PACK_SHRINK);
        row->pack_start(*Gtk::make_managed<Gtk::Label>("Posicionamento:"), Gtk::PACK_SHRINK);
        mode_.append("fit", "Ajustar à tela (sem recortar)");
        mode_.append("zoom", "Preencher a tela (recortar)");
        mode_.append("stretch", "Esticar");
        mode_.append("center", "Centralizar (tamanho original)");
        mode_.append("tile", "Mosaico (tile)");
        mode_.set_active_id("tile");
        mode_.set_tooltip_text("Ajustar mantém a imagem inteira; preencher recorta as bordas. "
                              "Esticar altera as proporções; centralizar e mosaico mantêm o tamanho original.");
        row->pack_start(mode_, Gtk::PACK_EXPAND_WIDGET);
        row->pack_start(*Gtk::make_managed<Gtk::Label>("Cor de fundo:"), Gtk::PACK_SHRINK);
        color_.set_use_alpha(false);
        color_.set_rgba(Gdk::RGBA("#000000"));
        color_.set_title("Cor de fundo do papel de parede");
        color_.set_tooltip_text("Cor das margens e das áreas transparentes da imagem.");
        row->pack_start(color_, Gtk::PACK_SHRINK);
        if (monitor_tools) {
            monitor_button_.set_label("Configurar fundo por monitor…");
            pack_start(monitor_button_, Gtk::PACK_SHRINK);
            monitor_button_.signal_clicked().connect(sigc::mem_fun(*this, &PlacementControls::editMonitors));
        }
    }

    backdrops::Selection selection(const fs::path& source) const {
        const auto color = color_.get_rgba();
        std::ostringstream hex;
        hex << '#' << std::hex << std::setfill('0')
            << std::setw(2) << color.get_red_u() / 257
            << std::setw(2) << color.get_green_u() / 257
            << std::setw(2) << color.get_blue_u() / 257;
        return {source, mode_.get_active_id(), hex.str(), monitors_};
    }

    void restore(const backdrops::Selection& selected) {
        monitors_ = selected.monitors;
        updateMonitorLabel();
        mode_.set_active_id(selected.mode);
        color_.set_rgba(Gdk::RGBA(selected.color));
    }

    std::function<fs::path()> sourcePath;

private:
    Gtk::Button monitor_button_;
    std::vector<backdrops::MonitorSelection> monitors_;

    void updateMonitorLabel() {
        monitor_button_.set_label(monitors_.empty() ? "Configurar fundo por monitor…" :
            "Fundos por monitor (" + std::to_string(monitors_.size()) + ")…");
    }

    void editMonitors() {
        auto* parent = dynamic_cast<Gtk::Window*>(get_toplevel());
        if (!parent) return;
        try {
            const auto layout = wallpaper::detectLayout();
            Gtk::Dialog dialog("Papel de parede por monitor", *parent, true);
            dialog.set_default_size(780, 560);
            dialog.add_button("Usar imagem única", Gtk::RESPONSE_REJECT);
            dialog.add_button("Cancelar", Gtk::RESPONSE_CANCEL);
            dialog.add_button("Usar estes fundos", Gtk::RESPONSE_OK);
            Gtk::ScrolledWindow scroll;
            scroll.set_policy(Gtk::POLICY_AUTOMATIC, Gtk::POLICY_AUTOMATIC);
            Gtk::Box rows(Gtk::ORIENTATION_VERTICAL, 12);
            rows.set_border_width(12);
            scroll.add(rows);
            dialog.get_content_area()->pack_start(scroll);
            Gtk::Label info("A orientação e a resolução vêm do monitor. A rotação manual gira apenas a imagem.\n"
                            "Monitores espelhados compartilham o mesmo fundo. As escolhas serão salvas em Aplicar.");
            info.set_line_wrap(true);
            rows.pack_start(info, Gtk::PACK_SHRINK);
            struct Row {
                std::string name;
                Gtk::FileChooserButton* file;
                PlacementControls* placement;
                Gtk::ComboBoxText* rotation;
            };
            std::vector<Row> controls;
            const auto base = selection(sourcePath ? sourcePath() : fs::path{});
            for (const auto& monitor : layout.monitors) {
                auto* frame = Gtk::make_managed<Gtk::Frame>(monitor.name + " — " +
                    std::to_string(monitor.width) + "×" + std::to_string(monitor.height) +
                    (monitor.height > monitor.width ? " · vertical" : " · horizontal") +
                    " · " + std::to_string(monitor.rotation) + "°");
                auto* box = Gtk::make_managed<Gtk::Box>(Gtk::ORIENTATION_VERTICAL, 6);
                box->set_border_width(10);
                frame->add(*box);
                rows.pack_start(*frame, Gtk::PACK_SHRINK);
                auto* file = Gtk::make_managed<Gtk::FileChooserButton>("Selecionar imagem", Gtk::FILE_CHOOSER_ACTION_OPEN);
                auto filter = Gtk::FileFilter::create();
                filter->set_name("Imagens"); filter->add_pixbuf_formats(); file->add_filter(filter);
                auto* placement = Gtk::make_managed<PlacementControls>(false);
                auto* rotation = Gtk::make_managed<Gtk::ComboBoxText>();
                rotation->append("auto", "Automática — respeitar orientação do monitor");
                rotation->append("0", "Imagem sem giro (0°)");
                rotation->append("90", "Girar imagem 90° à direita");
                rotation->append("180", "Girar imagem 180°");
                rotation->append("270", "Girar imagem 90° à esquerda");
                auto found = std::find_if(monitors_.begin(), monitors_.end(), [&](const auto& m) { return m.name == monitor.name; });
                const backdrops::MonitorSelection selected = found != monitors_.end() ? *found :
                    backdrops::MonitorSelection{monitor.name, base.source, base.mode, base.color, "auto"};
                if (!selected.source.empty()) file->set_filename(selected.source.string());
                placement->restore({selected.source, selected.mode, selected.color});
                rotation->set_active_id(selected.rotation);
                box->pack_start(*file, Gtk::PACK_SHRINK);
                box->pack_start(*placement, Gtk::PACK_SHRINK);
                box->pack_start(*rotation, Gtk::PACK_SHRINK);
                controls.push_back({monitor.name, file, placement, rotation});
            }
            dialog.show_all_children();
            for (;;) {
                const int response = dialog.run();
                if (response == Gtk::RESPONSE_REJECT) { monitors_.clear(); break; }
                if (response != Gtk::RESPONSE_OK) break;
                auto updated = monitors_; // Keep disconnected outputs for their next connection.
                bool complete = true;
                for (const auto& row : controls) {
                    const auto selected = row.placement->selection(row.file->get_filename());
                    if (selected.source.empty()) { complete = false; break; }
                    backdrops::MonitorSelection entry{row.name, selected.source, selected.mode, selected.color, row.rotation->get_active_id()};
                    auto found = std::find_if(updated.begin(), updated.end(), [&](const auto& m) { return m.name == row.name; });
                    if (found == updated.end()) updated.push_back(entry); else *found = entry;
                }
                if (complete) { monitors_ = std::move(updated); break; }
                Gtk::MessageDialog message(dialog, "Selecione uma imagem para cada monitor.", false, Gtk::MESSAGE_ERROR);
                message.run();
            }
            updateMonitorLabel();
        } catch (const std::exception& error) {
            Gtk::MessageDialog message(*parent, error.what(), false, Gtk::MESSAGE_ERROR);
            message.run();
        }
    }

    Gtk::ComboBoxText mode_;
    Gtk::ColorButton color_;
};

class MainWindow : public Gtk::Window {
public:
    MainWindow()
        : root_box_(Gtk::ORIENTATION_VERTICAL, 12),
          mode_box_(Gtk::ORIENTATION_VERTICAL, 6),
          all_box_(Gtk::ORIENTATION_VERTICAL, 6),
          one_box_(Gtk::ORIENTATION_VERTICAL, 6),
          four_grid_(),
          buttons_box_(Gtk::ORIENTATION_HORIZONTAL, 8),
          radio_all_("Aplicar uma imagem em todos os workspaces"),
          radio_one_("Aplicar imagem em um workspace específico"),
          radio_four_("Aplicar quatro imagens diferentes"),
          choose_all_button_("Selecionar imagem"),
          choose_one_button_("Selecionar imagem"),
          apply_button_("Aplicar"),
          quit_button_("Sair"),
          all_path_label_("Nenhuma imagem selecionada"),
          one_path_label_("Nenhuma imagem selecionada") {
        set_title("Configurar Fundo do EMWM");
        set_default_size(850, 600);
        set_border_width(16);

        const auto layout = wallpaper::detectLayout();
        resolution_ = {layout.width, layout.height};
        buildInterface();
        show_all_children();
        updateModeVisibility();
        try {
            saved_selections_ = backdrops::readSelections(getenvRequired("HOME"));
            restoreSelections();
        } catch (const std::exception& ex) {
            showError("Erro ao carregar papéis de parede", ex.what());
        }
    }

private:
    enum class Mode {
        All,
        One,
        Four
    };

    Gtk::Box root_box_;
    Gtk::ScrolledWindow scroll_;
    Gtk::Box panels_{Gtk::ORIENTATION_VERTICAL, 12};
    Gtk::Box mode_box_;
    Gtk::Box all_box_;
    Gtk::Box one_box_;
    Gtk::Grid four_grid_;
    Gtk::Box buttons_box_;
    Gtk::RadioButton radio_all_;
    Gtk::RadioButton radio_one_;
    Gtk::RadioButton radio_four_;
    Gtk::Button choose_all_button_;
    Gtk::Button choose_one_button_;
    std::array<Gtk::Button*, kWorkspaceCount> choose_four_buttons_{};
    Gtk::Button apply_button_;
    Gtk::Button quit_button_;
    Gtk::Label resolution_label_;
    Gtk::Label all_path_label_;
    Gtk::Label one_path_label_;
    std::array<Gtk::Label*, kWorkspaceCount> four_path_labels_{};
    Gtk::ComboBoxText workspace_combo_;
    PlacementControls all_placement_;
    PlacementControls one_placement_;
    std::array<PlacementControls*, kWorkspaceCount> four_placements_{};
    backdrops::Selections saved_selections_{};

    std::pair<int, int> resolution_{1920, 1080};
    fs::path all_image_;
    fs::path one_image_;
    std::array<fs::path, kWorkspaceCount> four_images_;

    void buildInterface() {
        add(root_box_);

        resolution_label_.set_text("Área total da tela: " + resolutionText(resolution_));
        resolution_label_.set_xalign(0.0f);
        root_box_.pack_start(resolution_label_, Gtk::PACK_SHRINK);
        auto* note = Gtk::make_managed<Gtk::Label>(
            "Escolha uma imagem única ou configure fundos por monitor. O desktop adapta os fundos ao conectar ou desligar monitores.");
        note->set_xalign(0.0f);
        note->set_line_wrap(true);
        root_box_.pack_start(*note, Gtk::PACK_SHRINK);

        auto group = radio_all_.get_group();
        radio_one_.set_group(group);
        radio_four_.set_group(group);
        radio_all_.set_active(true);

        mode_box_.pack_start(radio_all_, Gtk::PACK_SHRINK);
        mode_box_.pack_start(radio_one_, Gtk::PACK_SHRINK);
        mode_box_.pack_start(radio_four_, Gtk::PACK_SHRINK);
        scroll_.set_policy(Gtk::POLICY_AUTOMATIC, Gtk::POLICY_AUTOMATIC);
        scroll_.add(panels_);
        root_box_.pack_start(scroll_, Gtk::PACK_EXPAND_WIDGET);
        panels_.pack_start(mode_box_, Gtk::PACK_SHRINK);

        radio_all_.signal_toggled().connect(sigc::mem_fun(*this, &MainWindow::updateModeVisibility));
        radio_one_.signal_toggled().connect(sigc::mem_fun(*this, &MainWindow::updateModeVisibility));
        radio_four_.signal_toggled().connect(sigc::mem_fun(*this, &MainWindow::updateModeVisibility));

        buildAllPanel();
        buildOnePanel();
        buildFourPanel();

        panels_.pack_start(all_box_, Gtk::PACK_SHRINK);
        panels_.pack_start(one_box_, Gtk::PACK_SHRINK);
        panels_.pack_start(four_grid_, Gtk::PACK_SHRINK);

        buttons_box_.set_halign(Gtk::ALIGN_END);
        buttons_box_.pack_start(apply_button_, Gtk::PACK_SHRINK);
        buttons_box_.pack_start(quit_button_, Gtk::PACK_SHRINK);
        root_box_.pack_end(buttons_box_, Gtk::PACK_SHRINK);

        apply_button_.signal_clicked().connect(sigc::mem_fun(*this, &MainWindow::onApply));
        quit_button_.signal_clicked().connect(sigc::mem_fun(*this, &MainWindow::hide));
    }

    void buildAllPanel() {
        Gtk::Label* title = Gtk::make_managed<Gtk::Label>("Imagem para todos os workspaces");
        title->set_xalign(0.0f);
        all_path_label_.set_xalign(0.0f);
        all_path_label_.set_line_wrap(true);

        all_box_.pack_start(*title, Gtk::PACK_SHRINK);
        all_box_.pack_start(choose_all_button_, Gtk::PACK_SHRINK);
        all_box_.pack_start(all_path_label_, Gtk::PACK_SHRINK);
        all_box_.pack_start(all_placement_, Gtk::PACK_SHRINK);
        all_placement_.sourcePath = [this] { return all_image_; };

        choose_all_button_.signal_clicked().connect([this]() {
            if (auto selected = chooseImageFile()) {
                all_image_ = *selected;
                all_path_label_.set_text(all_image_.string());
            }
        });
    }

    void buildOnePanel() {
        Gtk::Label* title = Gtk::make_managed<Gtk::Label>("Imagem para um workspace específico");
        title->set_xalign(0.0f);
        one_path_label_.set_xalign(0.0f);
        one_path_label_.set_line_wrap(true);

        for (int i = 1; i <= kWorkspaceCount; ++i) {
            workspace_combo_.append(std::to_string(i), "Workspace " + std::to_string(i));
        }
        workspace_combo_.set_active_id("1");

        one_box_.pack_start(*title, Gtk::PACK_SHRINK);
        one_box_.pack_start(workspace_combo_, Gtk::PACK_SHRINK);
        one_box_.pack_start(choose_one_button_, Gtk::PACK_SHRINK);
        one_box_.pack_start(one_path_label_, Gtk::PACK_SHRINK);
        one_box_.pack_start(one_placement_, Gtk::PACK_SHRINK);
        one_placement_.sourcePath = [this] { return one_image_; };
        workspace_combo_.signal_changed().connect(sigc::mem_fun(*this, &MainWindow::restoreOneSelection));

        choose_one_button_.signal_clicked().connect([this]() {
            if (auto selected = chooseImageFile()) {
                one_image_ = *selected;
                one_path_label_.set_text(one_image_.string());
            }
        });
    }

    void buildFourPanel() {
        four_grid_.set_row_spacing(8);
        four_grid_.set_column_spacing(8);

        for (int i = 0; i < kWorkspaceCount; ++i) {
            Gtk::Label* workspace_label = Gtk::make_managed<Gtk::Label>("Workspace " + std::to_string(i + 1));
            workspace_label->set_xalign(0.0f);

            auto* button = Gtk::make_managed<Gtk::Button>("Selecionar imagem");
            auto* path_label = Gtk::make_managed<Gtk::Label>("Nenhuma imagem selecionada");
            path_label->set_xalign(0.0f);
            path_label->set_line_wrap(true);

            choose_four_buttons_[i] = button;
            four_path_labels_[i] = path_label;

            four_grid_.attach(*workspace_label, 0, 2 * i, 1, 1);
            four_grid_.attach(*button, 1, 2 * i, 1, 1);
            four_grid_.attach(*path_label, 2, 2 * i, 1, 1);
            four_placements_[i] = Gtk::make_managed<PlacementControls>();
            four_placements_[i]->sourcePath = [this, i] { return four_images_[i]; };
            four_grid_.attach(*four_placements_[i], 0, 2 * i + 1, 3, 1);

            button->signal_clicked().connect([this, i]() {
                if (auto selected = chooseImageFile()) {
                    four_images_[i] = *selected;
                    four_path_labels_[i]->set_text(four_images_[i].string());
                }
            });
        }
    }

    void restoreOneSelection() {
        const int workspace = workspace_combo_.get_active_row_number();
        if (workspace < 0) return;
        const auto selected = saved_selections_[workspace].value_or(backdrops::Selection{});
        one_image_ = selected.source;
        one_path_label_.set_text(one_image_.empty() ? "Nenhuma imagem selecionada" : one_image_.string());
        one_placement_.restore(selected);
    }

    void restoreSelections() {
        if (saved_selections_[0]) {
            all_image_ = saved_selections_[0]->source;
            all_path_label_.set_text(all_image_.string());
            all_placement_.restore(*saved_selections_[0]);
        }
        restoreOneSelection();
        for (int i = 0; i < kWorkspaceCount; ++i) {
            if (!saved_selections_[i]) continue;
            four_images_[i] = saved_selections_[i]->source;
            four_path_labels_[i]->set_text(four_images_[i].string());
            four_placements_[i]->restore(*saved_selections_[i]);
        }
    }

    std::optional<fs::path> chooseImageFile() {
        Gtk::FileChooserDialog dialog(*this, "Selecionar imagem", Gtk::FILE_CHOOSER_ACTION_OPEN);
        dialog.add_button("_Cancelar", Gtk::RESPONSE_CANCEL);
        dialog.add_button("_Selecionar", Gtk::RESPONSE_OK);

        auto filter = Gtk::FileFilter::create();
        filter->set_name("Imagens aceitas");
        filter->add_pattern("*.jpg");
        filter->add_pattern("*.jpeg");
        filter->add_pattern("*.png");
        filter->add_pattern("*.bmp");
        filter->add_pattern("*.webp");
        filter->add_pattern("*.tiff");
        filter->add_pattern("*.tif");
        filter->add_pattern("*.xpm");
        filter->add_pattern("*.JPG");
        filter->add_pattern("*.JPEG");
        filter->add_pattern("*.PNG");
        filter->add_pattern("*.BMP");
        filter->add_pattern("*.WEBP");
        filter->add_pattern("*.TIFF");
        filter->add_pattern("*.TIF");
        filter->add_pattern("*.XPM");
        dialog.add_filter(filter);

        if (dialog.run() == Gtk::RESPONSE_OK) {
            return fs::path(dialog.get_filename());
        }

        return std::nullopt;
    }

    Mode currentMode() const {
        if (radio_one_.get_active()) {
            return Mode::One;
        }
        if (radio_four_.get_active()) {
            return Mode::Four;
        }
        return Mode::All;
    }

    void updateModeVisibility() {
        const Mode mode = currentMode();
        all_box_.set_visible(mode == Mode::All);
        one_box_.set_visible(mode == Mode::One);
        four_grid_.set_visible(mode == Mode::Four);
    }

    ApplyPlan buildApplyPlan() const {
        ApplyPlan plan;

        switch (currentMode()) {
            case Mode::All: {

                for (int workspace = 0; workspace < kWorkspaceCount; ++workspace) {
                    plan.workspaces.push_back(workspace);
                    plan.selections.push_back(all_placement_.selection(all_image_));
                }
                break;
            }
            case Mode::One: {

                int workspace = std::stoi(std::string(workspace_combo_.get_active_id())) - 1;
                plan.workspaces.push_back(workspace);
                plan.selections.push_back(one_placement_.selection(one_image_));
                break;
            }
            case Mode::Four: {
                for (int workspace = 0; workspace < kWorkspaceCount; ++workspace) {

                    plan.workspaces.push_back(workspace);
                    plan.selections.push_back(four_placements_[workspace]->selection(four_images_[workspace]));
                }
                break;
            }
        }

        return plan;
    }

    void onApply() {
        try {
            const fs::path home = getenvRequired("HOME");
            ApplyPlan plan = buildApplyPlan();
            const auto layout = wallpaper::detectLayout();
            resolution_ = {layout.width, layout.height};
            resolution_label_.set_text("Área total da tela: " + resolutionText(resolution_));
            std::map<std::string, fs::path> prepared;
            for (const auto& selected : plan.selections) {
                if (selected.monitors.empty()) ensureReadableImage(selected.source);
                std::ostringstream key;
                key << std::quoted(selected.source.string()) << selected.mode << selected.color;
                for (const auto& monitor : selected.monitors)
                    key << std::quoted(monitor.name) << std::quoted(monitor.source.string())
                        << monitor.mode << monitor.color << monitor.rotation;
                auto found = prepared.find(key.str());
                if (found == prepared.end()) {
                    const auto output = wallpaper::prepare(home, selected, layout);
                    found = prepared.emplace(key.str(), output).first;
                }
                plan.outputs.push_back(found->second);
            }

            const auto resources = backdrops::save(home, plan.workspaces, plan.outputs, plan.selections);
            saved_selections_ = backdrops::readSelections(home);
            restoreSelections();
            const std::string merge_warning = mergeBackdropResources(resources);

            for (size_t i = 0; i < plan.workspaces.size(); ++i) {
                appendDebugLog(
                    "Fundo salvo ws" + std::to_string(plan.workspaces[i]) +
                    "=" + fs::absolute(plan.outputs[i]).string()
                );
            }
            const TellmwmResult tellmwm_result = applyWithTellmwm(plan.workspaces, plan.outputs);

            std::string success_message =
                "Os fundos foram salvos e serão restaurados ao entrar na sessão EMWM.\n";

            if (tellmwm_result.all_applied) {
                success_message += "O fundo foi aplicado imediatamente com tellmwm.";
            } else if (tellmwm_result.command_found) {
                success_message += "Não foi possível enviar todos os fundos ao tellmwm.\n";
                success_message += tellmwm_result.details;
            } else {
                success_message += tellmwm_result.details;
            }

            if (!merge_warning.empty()) success_message += "\n" + merge_warning;

            showInfo(
                "Concluído",
                success_message
            );
        } catch (const Glib::Error& ex) {
            showError("Erro ao aplicar papel de parede", ex.what());
        } catch (const std::exception& ex) {
            showError("Erro ao aplicar papel de parede", ex.what());
        }
    }

    void showError(const Glib::ustring& title, const Glib::ustring& message) {
        Gtk::MessageDialog dialog(*this, title, false, Gtk::MESSAGE_ERROR, Gtk::BUTTONS_OK, true);
        dialog.set_secondary_text(message);
        dialog.run();
    }

    void showInfo(const Glib::ustring& title, const Glib::ustring& message) {
        Gtk::MessageDialog dialog(*this, title, false, Gtk::MESSAGE_INFO, Gtk::BUTTONS_OK, true);
        dialog.set_secondary_text(message);
        dialog.run();
    }
};

} // namespace

int main(int argc, char* argv[]) {
    if (argc == 2 && std::string(argv[1]) == "--refresh-layout") {
        try {
            const fs::path home = getenvRequired("HOME");
            const auto selections = backdrops::readSelections(home);
            const auto saved = backdrops::readSaved(home);
            const auto layout = wallpaper::detectLayout();
            std::vector<int> workspaces;
            std::vector<fs::path> outputs;
            std::vector<backdrops::Selection> choices;
            std::map<std::string, fs::path> prepared;
            for (int i = 0; i < backdrops::workspaceCount; ++i) {
                if (!selections[i]) continue;
                const auto& selected = *selections[i];
                std::ostringstream key;
                key << std::quoted(selected.source.string()) << selected.mode << selected.color;
                for (const auto& monitor : selected.monitors)
                    key << std::quoted(monitor.name) << std::quoted(monitor.source.string())
                        << monitor.mode << monitor.color << monitor.rotation;
                auto found = prepared.find(key.str());
                if (found == prepared.end())
                    found = prepared.emplace(key.str(), wallpaper::prepare(home, selected, layout)).first;
                if (saved[i] && *saved[i] == found->second) continue;
                workspaces.push_back(i); outputs.push_back(found->second); choices.push_back(selected);
            }
            if (!workspaces.empty()) {
                const auto resources = backdrops::save(home, workspaces, outputs, choices);
                const auto warning = mergeBackdropResources(resources);
                if (!warning.empty()) throw std::runtime_error(warning);
                applyWithTellmwm(workspaces, outputs);
            }
            return 0;
        } catch (const std::exception& ex) {
            std::cerr << "Atualização dos fundos: " << ex.what() << '\n';
            return 1;
        }
    }
    if (argc == 2 && std::string(argv[1]) == "--migrate") {
        try {
            const auto resources = backdrops::save(getenvRequired("HOME"), {}, {});
            std::cout << "Fundos salvos em " << resources << '\n';
            const auto warning = mergeBackdropResources(resources);
            if (!warning.empty()) {
                std::cerr << warning << '\n';
                return 1;
            }
            return 0;
        } catch (const std::exception& ex) {
            std::cerr << ex.what() << '\n';
            return 1;
        }
    }
    auto app = Gtk::Application::create(argc, argv, "br.com.paulo.emwm-wallpaper-tool");
    MainWindow window;
    return app->run(window);
}
