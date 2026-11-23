// Copyright (C) 2018–2026 alx@fastestcode.org. SPDX-License-Identifier: MIT
//! Argument quoting accepted by manual Toolchest menu entries.
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

pub(crate) fn split_command(input: &[u8]) -> Result<Vec<OsString>, ()> {
    let mut words = Vec::new();
    let mut rest = input.iter().copied().peekable();
    while rest.peek().is_some() {
        while rest.next_if(|b| matches!(b, b' ' | b'\t')).is_some() {}
        if rest.peek().is_none() {
            break;
        }
        let mut word = Vec::new();
        let mut previous = 0;
        while let Some(mut b) = rest.next() {
            if matches!(b, b'\'' | b'"') {
                if previous == b'\\' {
                    word.pop();
                    word.push(b);
                } else {
                    // Both quote characters terminate a quoted span, as in toolboxrc.
                    loop {
                        match rest.next() {
                            Some(b'\'' | b'"') => break,
                            Some(c) => word.push(c),
                            None => return Err(()),
                        }
                    }
                }
                // The byte after a literal or closing quote is consumed as text.
                let Some(next) = rest.next() else { break };
                b = next;
            }
            if matches!(b, b' ' | b'\t') {
                break;
            }
            word.push(b);
            previous = b;
        }
        words.push(OsString::from_vec(word));
    }
    if words.is_empty() {
        return Err(());
    }
    Ok(words)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn command_words() {
        assert_eq!(
            split_command(br#"run "a b" ''"#).unwrap(),
            ["run", "a b", ""]
        );
        assert_eq!(split_command(b"run 'bad"), Err(()));
    }
}
