#include "backdrops.h"

#include <X11/Xresource.h>
#include <sys/stat.h>
#include <unistd.h>

#include <cerrno>
#include <cctype>
#include <fstream>
#include <memory>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <type_traits>

namespace fs = std::filesystem;
namespace backdrops {
namespace {
std::string readFile(const fs::path& path) {
    std::ifstream input(path, std::ios::binary);
    if (!input) throw std::runtime_error("Não foi possível ler " + path.string());
    std::ostringstream data;
    data << input.rdbuf();
    if (input.bad()) throw std::runtime_error("Falha ao ler " + path.string());
    return data.str();
}

// Follow existing symlinks, preserve permissions and replace only complete files.
void writeAtomic(fs::path path, const std::string& contents, bool backup) {
    if (fs::is_symlink(path)) path = fs::canonical(path);
    const bool exists = fs::exists(path);
    if (exists && readFile(path) == contents) return;
    if (exists && backup) {
        fs::copy_file(path, path.string() + ".before-wallpaper-resources",
                      fs::copy_options::skip_existing);
    }
    fs::create_directories(path.parent_path());
    struct stat st{};
    const mode_t mode = exists && ::stat(path.c_str(), &st) == 0 ? st.st_mode & 0777 : 0600;
    std::string name = path.string() + ".tmp.XXXXXX";
    int fd = ::mkstemp(name.data());
    if (fd < 0) throw std::runtime_error("Não foi possível criar arquivo temporário para " + path.string());
    try {
        if (::fchmod(fd, mode) != 0) throw std::runtime_error("Falha ao preservar permissões.");
        size_t offset = 0;
        while (offset < contents.size()) {
            const ssize_t count = ::write(fd, contents.data() + offset, contents.size() - offset);
            if (count < 0 && errno == EINTR) continue;
            if (count <= 0) throw std::runtime_error("Falha ao gravar " + path.string());
            offset += static_cast<size_t>(count);
        }
        if (::fsync(fd) != 0) throw std::runtime_error("Falha ao sincronizar " + path.string());
        const int result = ::close(fd);
        fd = -1;
        if (result != 0) throw std::runtime_error("Falha ao fechar " + path.string());
        fs::rename(name, path);
    } catch (...) {
        if (fd >= 0) ::close(fd);
        ::unlink(name.c_str());
        throw;
    }
}

std::string encodeImage(const fs::path& path) {
    const std::string name = fs::absolute(path).string();
    // EMWM's GetString strips trailing whitespace even inside quotes.
    if (name.empty() || std::isspace(static_cast<unsigned char>(name.back())) ||
        name.find_first_of("\n\r") != std::string::npos || name.find('\0') != std::string::npos) {
        throw std::runtime_error("O caminho da imagem não pode conter quebras de linha nem terminar em espaço: " + name);
    }
    std::string value = "\"";
    for (char c : name) {
        // Xrm decodes octal escapes; GetString then decodes Motif quoting.
        // Octal keeps the C preprocessor used by xrdb from closing our string.
        if (c == '\\') value += "\\134\\134";
        else if (c == '"') value += "\\134\\042";
        else value += c;
    }
    return value + '"';
}

std::string decodeImage(const std::string& value) {
    size_t i = value.find_first_not_of(" \t");
    if (i == std::string::npos) throw std::runtime_error("Recurso de fundo vazio.");
    const bool quoted = value[i] == '"';
    if (quoted) ++i;
    std::string path;
    bool closed = !quoted;
    for (; i < value.size(); ++i) {
        const char c = value[i];
        if (quoted && c == '"') { closed = true; break; }
        if (!quoted && (std::isspace(static_cast<unsigned char>(c)) || c == '#')) break;
        if (c == '\\') {
            if (++i == value.size()) throw std::runtime_error("Escape incompleto no recurso de fundo.");
            path += value[i];
        } else path += c;
    }
    if (!closed || path.empty()) throw std::runtime_error("Recurso de fundo inválido.");
    return path;
}

Paths readLegacy(const fs::path& path) {
    Paths paths{};
    if (!fs::exists(path)) return paths;
    std::istringstream input(readFile(path));
    // Read data only. Never execute the legacy script.
    const std::regex pattern(R"(^\s*(?:/usr/bin/)?tellmwm\s+backdrop\s+ws([0-3])\s+(.+?)\s*$)");
    std::string line;
    while (std::getline(input, line)) {
        std::smatch match;
        if (!std::regex_match(line, match, pattern)) continue;
        std::string path = match[2];
        if (path.size() >= 2 && path.front() == '\'' && path.back() == '\'') {
            path = path.substr(1, path.size() - 2);
            path = std::regex_replace(path, std::regex(R"('\\'')"), "'");
        }
        paths[std::stoi(match[1])] = path;
    }
    return paths;
}
}

fs::path resourceFile(const fs::path& home) {
    // Keep the existing tool's configuration location for backwards compatibility.
    return fs::absolute(home) / ".config/emwm/backdrops.resources";
}

std::string tellmwmImageArgument(const fs::path& image) {
    // tellmwm wraps the argument in quotes without escaping it. The WM then
    // tokenizes both the IPC request and the backdrop value. Quote both layers.
    std::string motif = "\"";
    for (char c : fs::absolute(image).string()) {
        if (c == '\\' || c == '"') motif += '\\';
        motif += c;
    }
    motif += '"';
    std::string ipc;
    for (char c : motif) {
        if (c == '\\' || c == '"') ipc += '\\';
        ipc += c;
    }
    return ipc;
}

Paths readSaved(const fs::path& home) {
    const auto file = resourceFile(home);
    if (!fs::exists(file)) return readLegacy(home / ".config/emwm/apply-backdrops.sh");
    const std::string contents = readFile(file);
    XrmInitialize();
    std::unique_ptr<std::remove_pointer_t<XrmDatabase>, decltype(&XrmDestroyDatabase)>
        database(XrmGetStringDatabase(contents.c_str()), XrmDestroyDatabase);
    if (!database) throw std::runtime_error("Não foi possível interpretar " + file.string());
    Paths paths{};
    for (int i = 0; i < workspaceCount; ++i) {
        const std::string key = "Emwm.ws" + std::to_string(i) + ".backdrop.image";
        XrmValue value{};
        char* type = nullptr;
        if (XrmGetResource(database.get(), key.c_str(), key.c_str(), &type, &value))
            paths[i] = decodeImage(value.addr);
    }
    return paths;
}

fs::path save(const fs::path& home, const std::vector<int>& workspaces,
              const std::vector<fs::path>& images) {
    if (workspaces.size() != images.size()) throw std::runtime_error("Plano de aplicação inválido.");
    auto paths = readSaved(home);
    for (size_t i = 0; i < workspaces.size(); ++i) {
        if (workspaces[i] < 0 || workspaces[i] >= workspaceCount)
            throw std::runtime_error("Workspace inválido.");
        paths[workspaces[i]] = fs::absolute(images[i]);
    }
    std::ostringstream resources;
    resources << "! Gerenciado pelo emwm-wallpaper-tool.\n";
    bool found = false;
    for (int i = 0; i < workspaceCount; ++i) {
        if (!paths[i]) continue;
        found = true;
        resources << "Emwm*ws" << i << "*backdrop*image: " << encodeImage(*paths[i]) << '\n';
    }
    if (!found) throw std::runtime_error("Nenhum fundo salvo para migrar.");
    const fs::path file = resourceFile(home);
    if (file.string().find_first_of("\"\\\n\r") != std::string::npos)
        throw std::runtime_error("O caminho HOME não pode conter aspas, barras invertidas ou quebras de linha.");
    const fs::path xresources = fs::absolute(home) / ".Xresources";
    std::string config = fs::exists(xresources) ? readFile(xresources) : "";
    const std::string include = "#include \"" + file.string() + "\"";
    std::istringstream lines(config);
    std::string line;
    bool included = false;
    while (std::getline(lines, line)) {
        if (line == include) included = true;
    }
    if (!included) {
        if (!config.empty() && config.back() != '\n') config += '\n';
        config += "\n! Fundos dos workspaces do EMWM (emwm-wallpaper-tool).\n" + include + '\n';
    }
    // The include never points to a partially written/nonexistent resource file.
    writeAtomic(file, resources.str(), true);
    writeAtomic(xresources, config, true);
    return file;
}
}
