// SPDX-License-Identifier: GPL-3.0-only
use std::collections::HashSet;

pub const FIELDS: [&str; 5] = [
    "*background",
    "*foreground",
    "Emwm*client*background",
    "Emwm*client*activeBackground",
    "Emwm*icon*activeBackground",
];

pub fn palette(name: &str) -> Option<[&'static str; 5]> {
    Some(match name {
        "keep" | "custom" => ["#d2d2d2", "#000000", "#8c8c8c", "#7399ba", "#7399ba"],
        "irix" => ["#d2d2d2", "#000000", "#8c8c8c", "#aba68c", "#aba68c"],
        "classic" => ["#bdbdbd", "#000000", "#a0a0a0", "#7399ba", "#7399ba"],
        _ => return None,
    })
}

pub fn valid_name(text: &str) -> bool {
    !text.is_empty()
        && text.chars().count() <= 200
        && !matches!(text, "." | "..")
        && !text.chars().any(|c| c < ' ' || matches!(c, '/' | '\\'))
}

fn python_space(c: char) -> bool {
    c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c)
}

fn header(line: &str) -> Option<&str> {
    let rest = line.trim_start_matches(python_space).strip_prefix('[')?;
    let (section, _) = rest.split_once(']')?;
    (!section.is_empty()).then_some(section)
}

fn assignment(line: &str) -> Option<&str> {
    let fields = line.trim_start_matches(python_space);
    let end = fields.find(|c| python_space(c) || matches!(c, '#' | ';' | '='))?;
    (end > 0
        && fields[end..]
            .trim_start_matches(python_space)
            .starts_with('='))
    .then_some(&fields[..end])
}

pub struct Setting {
    pub key: String,
    pub lowercase: String,
    pub value: String,
}

fn append_setting(output: &mut String, setting: &Setting) {
    output.push_str(&setting.key);
    output.push('=');
    output.push_str(&setting.value);
    output.push('\n');
}

pub fn edit_ini(
    lines: impl IntoIterator<Item = impl AsRef<str>>,
    section: &str,
    settings: &[Setting],
) -> String {
    let mut current = String::new();
    let mut found = false;
    let mut seen = HashSet::new();
    let mut output = String::new();
    for line in lines {
        let line = line.as_ref();
        if let Some(new_section) = header(line) {
            if current == section {
                for setting in settings {
                    if !seen.contains(&setting.lowercase) {
                        append_setting(&mut output, setting);
                    }
                }
                seen.extend(settings.iter().map(|s| &s.lowercase));
            }
            current = new_section.to_owned();
            found |= current == section;
        }
        if let Some(key) = assignment(line).filter(|_| current == section) {
            let lower = key.to_lowercase();
            if let Some(setting) = settings.iter().rev().find(|s| s.lowercase == lower) {
                if seen.insert(&setting.lowercase) {
                    append_setting(&mut output, setting);
                }
                continue;
            }
        }
        output.push_str(line);
        if !line.ends_with('\n') {
            output.push('\n');
        }
    }
    if !found {
        output.push_str(&format!("\n[{section}]\n"));
    }
    for setting in settings {
        if !seen.contains(&setting.lowercase) {
            append_setting(&mut output, setting);
        }
    }
    output
}

fn line_prefix(haystack: &str, needle: &str) -> Option<usize> {
    haystack
        .match_indices(needle)
        .find(|(at, _)| *at == 0 || haystack.as_bytes()[at - 1] == b'\n')
        .map(|(at, _)| at)
}

pub fn managed_block(text: &str, name: &str, content: &str, comment: &str) -> String {
    let start = format!("{comment} BEGIN {name}\n");
    let end = format!("{comment} END {name}");
    let mut remaining = text;
    let mut output = String::new();
    while let Some(open) = line_prefix(remaining, &start) {
        let after = &remaining[open + start.len()..];
        let Some(close) = line_prefix(after, &end) else {
            break;
        };
        output.push_str(&remaining[..open]);
        let suffix = &after[close + end.len()..];
        remaining = suffix.strip_prefix('\n').unwrap_or(suffix);
    }
    output.push_str(remaining);
    output.truncate(output.trim_end_matches('\n').len());
    output.push_str("\n\n");
    output.push_str(&start);
    output.push_str(content.trim_end_matches('\n'));
    output.push('\n');
    output.push_str(&end);
    output.push('\n');
    output
}
