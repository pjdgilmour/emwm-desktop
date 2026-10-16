#include <gtkmm.h>
#include "backdrops.h"
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
#include <regex>
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
    std::vector<fs::path> inputs;
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

std::pair<int, int> detectScreenResolution() {
    if (!commandExists("xrandr")) {
        return {1920, 1080};
    }

    CommandResult result = runCommand({"xrandr", "--current"}, true);
    if (result.exit_code != 0) {
        return {1920, 1080};
    }

    std::regex active_mode(R"((\d+)x(\d+)\s+[^\n]*\*)");
    std::istringstream lines(result.output);
    std::string line;
    int best_width = 0;
    int best_height = 0;
    long long best_area = 0;

    while (std::getline(lines, line)) {
        std::smatch match;
        if (std::regex_search(line, match, active_mode)) {
            int width = std::stoi(match[1].str());
            int height = std::stoi(match[2].str());
            long long area = static_cast<long long>(width) * height;
            if (area > best_area) {
                best_width = width;
                best_height = height;
                best_area = area;
            }
        }
    }

    if (best_width <= 0 || best_height <= 0) {
        return {1920, 1080};
    }

    return {best_width, best_height};
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
        set_default_size(760, 520);
        set_border_width(16);

        resolution_ = detectScreenResolution();
        buildInterface();
        show_all_children();
        updateModeVisibility();
    }

private:
    enum class Mode {
        All,
        One,
        Four
    };

    Gtk::Box root_box_;
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

    std::pair<int, int> resolution_{1920, 1080};
    fs::path all_image_;
    fs::path one_image_;
    std::array<fs::path, kWorkspaceCount> four_images_;

    void buildInterface() {
        add(root_box_);

        resolution_label_.set_text("Resolução detectada: " + resolutionText(resolution_));
        resolution_label_.set_xalign(0.0f);
        root_box_.pack_start(resolution_label_, Gtk::PACK_SHRINK);

        auto group = radio_all_.get_group();
        radio_one_.set_group(group);
        radio_four_.set_group(group);
        radio_all_.set_active(true);

        mode_box_.pack_start(radio_all_, Gtk::PACK_SHRINK);
        mode_box_.pack_start(radio_one_, Gtk::PACK_SHRINK);
        mode_box_.pack_start(radio_four_, Gtk::PACK_SHRINK);
        root_box_.pack_start(mode_box_, Gtk::PACK_SHRINK);

        radio_all_.signal_toggled().connect(sigc::mem_fun(*this, &MainWindow::updateModeVisibility));
        radio_one_.signal_toggled().connect(sigc::mem_fun(*this, &MainWindow::updateModeVisibility));
        radio_four_.signal_toggled().connect(sigc::mem_fun(*this, &MainWindow::updateModeVisibility));

        buildAllPanel();
        buildOnePanel();
        buildFourPanel();

        root_box_.pack_start(all_box_, Gtk::PACK_SHRINK);
        root_box_.pack_start(one_box_, Gtk::PACK_SHRINK);
        root_box_.pack_start(four_grid_, Gtk::PACK_SHRINK);

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

            four_grid_.attach(*workspace_label, 0, i, 1, 1);
            four_grid_.attach(*button, 1, i, 1, 1);
            four_grid_.attach(*path_label, 2, i, 1, 1);

            button->signal_clicked().connect([this, i]() {
                if (auto selected = chooseImageFile()) {
                    four_images_[i] = *selected;
                    four_path_labels_[i]->set_text(four_images_[i].string());
                }
            });
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
                ensureReadableImage(all_image_);
                for (int workspace = 0; workspace < kWorkspaceCount; ++workspace) {
                    plan.workspaces.push_back(workspace);
                    plan.inputs.push_back(all_image_);
                    plan.outputs.push_back(all_image_);
                }
                break;
            }
            case Mode::One: {
                ensureReadableImage(one_image_);
                int workspace = std::stoi(std::string(workspace_combo_.get_active_id())) - 1;
                plan.workspaces.push_back(workspace);
                plan.inputs.push_back(one_image_);
                plan.outputs.push_back(one_image_);
                break;
            }
            case Mode::Four: {
                for (int workspace = 0; workspace < kWorkspaceCount; ++workspace) {
                    ensureReadableImage(four_images_[workspace]);
                    plan.workspaces.push_back(workspace);
                    plan.inputs.push_back(four_images_[workspace]);
                    plan.outputs.push_back(four_images_[workspace]);
                }
                break;
            }
        }

        return plan;
    }

    void onApply() {
        try {
            const fs::path home = getenvRequired("HOME");
            const ApplyPlan plan = buildApplyPlan();

            const auto resources = backdrops::save(home, plan.workspaces, plan.outputs);
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
