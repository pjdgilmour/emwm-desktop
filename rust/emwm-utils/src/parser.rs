// SPDX-License-Identifier: MIT
// Based on the toolbox configuration parser by alx@fastestcode.org.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum Kind {
    Cascade,
    Command,
    Separator,
}

pub struct Entry {
    pub kind: Kind,
    pub level: i32,
    pub mnemonic: u8,
    pub title: Vec<u8>,
    pub command: Vec<u8>,
}

fn blank(c: &u8) -> bool {
    matches!(c, b' ' | b'\t')
}

fn trim_left(bytes: &[u8]) -> &[u8] {
    let first = bytes.iter().position(|c| !blank(c)).unwrap_or(bytes.len());
    &bytes[first..]
}

fn entry(line: &[u8], level: i32) -> Entry {
    let mut e = Entry {
        kind: Kind::Cascade,
        level,
        mnemonic: 0,
        title: Vec::new(),
        command: Vec::new(),
    };
    if line == b"SEPARATOR" {
        e.kind = Kind::Separator;
        return e;
    }
    let mut rest = line;
    while let Some((&c, after)) = rest.split_first() {
        rest = after;
        match (c, rest.split_first()) {
            (b'\\', Some((&literal @ (b'\\' | b'&' | b':'), tail))) => {
                e.title.push(literal);
                rest = tail;
            }
            (b'&', next) => {
                e.mnemonic = next.map_or(0, |(&key, _)| key);
                if let Some((&key, tail)) = next {
                    e.title.push(key);
                    rest = tail;
                }
            }
            (b':', _) => {
                e.kind = Kind::Command;
                e.command = trim_left(rest).to_vec();
                break;
            }
            _ => e.title.push(c),
        }
    }
    e
}

pub fn parse(data: &[u8]) -> Result<Vec<Entry>, String> {
    // X11 configurations may contain non-UTF-8 filenames and command bytes.
    let data = data.split(|&c| c == 0).next().unwrap_or_default();
    let mut entries: Vec<Entry> = Vec::new();
    let mut level = 0;
    for (number, raw) in data.split(|&c| c == b'\n').enumerate() {
        let error = |message: &str| format!("Line {}: {message}", number + 1);
        let end = raw.iter().rposition(|c| !blank(c)).map_or(0, |end| end + 1);
        let line = trim_left(&raw[..end]);
        let previous = entries.last();
        match line.first() {
            None | Some(b'#') => continue,
            Some(b'{') => {
                if previous.is_none_or(|e| e.kind != Kind::Cascade) {
                    return Err(error("Delimiter '{' must follow a cascade entry"));
                }
                level += 1;
                continue;
            }
            Some(b'}') => {
                if level == 0 || previous.is_none_or(|e| e.kind != Kind::Command) {
                    return Err(error("Delimiter '}' out of scope"));
                }
                level -= 1;
                continue;
            }
            _ if previous.is_some_and(|e| e.kind == Kind::Cascade && e.level == level) => {
                return Err(error("Cascade entry must have a menu scope"));
            }
            _ => (),
        }
        let mut item = entry(line, level);
        match item.kind {
            Kind::Command if level == 0 => {
                return Err(error("Command entries must reside within a menu scope"));
            }
            Kind::Command if item.command.is_empty() => {
                return Err(error("Command string expected after ':' "));
            }
            Kind::Cascade if item.title.last() == Some(&b'{') => {
                item.title.pop();
                while item.title.len() > 1 && item.title.last().is_some_and(blank) {
                    item.title.pop();
                }
                level += 1;
            }
            _ => (),
        }
        entries.push(item);
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_menu_with_command_bytes_and_separators() -> Result<(), String> {
        let items =
            parse(b" &Menu {\nSEPARATOR\nsub\n{\nC\\:D\\&E &X:  run &\\:$HOME\xff\n}\n}\n")?;
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].title, b"Menu");
        assert_eq!(items[0].mnemonic, b'M');
        assert_eq!(items[1].kind, Kind::Separator);
        assert_eq!(items[2].level, 1);
        assert_eq!(items[3].level, 2);
        assert_eq!(items[3].title, b"C:D&E X");
        assert_eq!(items[3].command, b"run &\\:$HOME\xff");
        Ok(())
    }

    #[test]
    fn errors_and_incomplete_menus_follow_the_original() {
        assert_eq!(
            parse(b"menu\nother").err().as_deref(),
            Some("Line 2: Cascade entry must have a menu scope")
        );
        assert_eq!(
            parse(b"menu {\ncmd:\t").err().as_deref(),
            Some("Line 2: Command string expected after ':' ")
        );
        assert!(parse(b"menu {\nok: x\0\n{").is_ok());
    }
}
