#pragma once

#include <array>
#include <filesystem>
#include <optional>
#include <string>
#include <vector>

namespace backdrops {
constexpr int workspaceCount = 4;
using Paths = std::array<std::optional<std::filesystem::path>, workspaceCount>;

std::filesystem::path resourceFile(const std::filesystem::path& home);
std::string tellmwmImageArgument(const std::filesystem::path& image);
Paths readSaved(const std::filesystem::path& home);
// Empty updates migrate existing selections. Returns the file to merge with xrdb.
std::filesystem::path save(const std::filesystem::path& home,
                          const std::vector<int>& workspaces,
                          const std::vector<std::filesystem::path>& images);
}
