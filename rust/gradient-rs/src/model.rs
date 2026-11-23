// SPDX-License-Identifier: GPL-3.0-only
pub const COLORS: [(&str, &str, &str); 6] = [
    ("active_start", "ActiveStart", "#24466b"),
    ("active_end", "ActiveEnd", "#7399ba"),
    ("active_text", "ActiveText", "#ffffff"),
    ("inactive_start", "InactiveStart", "#b0b0b0"),
    ("inactive_end", "InactiveEnd", "#707070"),
    ("inactive_text", "InactiveText", "#101010"),
];

pub fn is_color(value: &str) -> bool {
    value.len() == 7
        && value.starts_with('#')
        && value.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
}

pub struct Gradient {
    pub mode: String,
    pub colors: Vec<String>,
}

impl Gradient {
    pub fn read<E>(
        mut lookup: impl FnMut(&str) -> Result<Option<String>, E>,
        error: impl Fn(&str) -> E,
    ) -> Result<Self, E> {
        let mode = lookup("mode")?.unwrap_or_else(|| "none".to_owned());
        if !matches!(
            mode.as_str(),
            "none" | "horizontal" | "vertical" | "diagonal"
        ) {
            return Err(error("Direção de gradiente inválida."));
        }
        let mut colors = Vec::with_capacity(COLORS.len());
        for (key, _, default) in COLORS {
            let value = lookup(key)?.unwrap_or_else(|| default.to_owned());
            if !is_color(&value) {
                return Err(error("Cor do gradiente inválida: use #RRGGBB."));
            }
            colors.push(value.to_ascii_lowercase());
        }
        Ok(Self { mode, colors })
    }

    pub fn resources(&self) -> String {
        let mut text = format!("Emwm*titleGradient: {}\n", self.mode);
        for ((_, suffix, _), color) in COLORS.iter().zip(&self.colors) {
            text.push_str("Emwm*titleGradient");
            text.push_str(suffix);
            text.push_str(": ");
            text.push_str(color);
            text.push('\n');
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::is_color;

    #[test]
    fn colors_are_exactly_six_ascii_hex_digits_after_hash() {
        assert!(is_color("#00AafF"));
        for value in ["", "123456", "#12345", "#abcdef\n", "#１２３", "#G00000"] {
            assert!(!is_color(value));
        }
    }
}
