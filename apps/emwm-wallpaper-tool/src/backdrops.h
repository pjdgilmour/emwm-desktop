#pragma once

#include <array>
#include <filesystem>
#include <optional>
#include <string>
#include <vector>

namespace backdrops {
constexpr int workspaceCount = 4;
using Paths = std::array<std::optional<std::filesystem::path>, workspaceCount>;
struct MonitorSelection {
    std::string name;
    std::filesystem::path source;
    std::string mode = "tile";
    std::string color = "#000000";
    std::string rotation = "auto";
};
struct Selection {
    std::filesystem::path source;
    std::string mode = "tile";
    std::string color = "#000000";
    std::vector<MonitorSelection> monitors = {};
};
using Selections = std::array<std::optional<Selection>, workspaceCount>;

bool validMode(const std::string& mode);
bool validColor(const std::string& color);
bool validRotation(const std::string& rotation);
Selections readSelections(const std::filesystem::path& home);

std::filesystem::path resourceFile(const std::filesystem::path& home);
std::string tellmwmImageArgument(const std::filesystem::path& image);
Paths readSaved(const std::filesystem::path& home);
// Empty updates migrate existing selections. Returns the file to merge with xrdb.
std::filesystem::path save(const std::filesystem::path& home,
                          const std::vector<int>& workspaces,
                          const std::vector<std::filesystem::path>& images,
                          const std::vector<Selection>& selections = {});
}
