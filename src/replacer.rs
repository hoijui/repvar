// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use dict::{Dict, DictIface};
use std::borrow::Cow;
use std::io::{self, BufRead, Write};

fn replacement(
    vars: &Dict<String>,
    key: &str,
    fail_on_missing: bool,
) -> io::Result<(bool, String)> {
    return match vars.get(key) {
        Some(val) => Ok((true, val.to_string())),
        None => {
            if fail_on_missing {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Undefined variable '{}'", key),
                ))
            } else {
                Ok((false, format!("${{{}}}", key)))
            }
        }
    };
}

enum ReplState {
    Text,
    Dollar1,
    Dollar2,
    Key,
}

/// Replaces all occurences of variables of the form `${KEY}` in a string
/// with their respective values.
///
/// # Errors
///
/// If a variable key was found in the stream,
/// but `vars` contains no entry for it,
/// and `fail_on_missing` is `true`.
pub fn replace_in_string<'t>(
    vars: &Dict<String>,
    line: &'t str,
    fail_on_missing: bool,
) -> io::Result<Cow<'t, str>> {
    let mut state = ReplState::Text;
    let mut key = String::with_capacity(64);
    let mut buff_text = String::with_capacity(line.len());
    let mut buff_special = String::with_capacity(5);
    let mut buff_out = String::with_capacity(line.len() * 3 / 2);
    let mut replaced = false;
    for chr in line.chars() {
        match state {
            ReplState::Text => {
                if chr == '$' {
                    state = ReplState::Dollar1;
                    buff_out.push_str(&buff_text);
                    buff_text.clear();
                    buff_special.push(chr);
                } else {
                    buff_text.push(chr);
                }
            }
            ReplState::Dollar1 => {
                if chr == '$' {
                    state = ReplState::Dollar2;
                    buff_special.push(chr);
                } else if chr == '{' {
                    state = ReplState::Key;
                    buff_special.clear();
                } else {
                    state = ReplState::Text;
                    buff_out.push_str(&buff_special);
                    buff_special.clear();
                }
            }
            ReplState::Dollar2 => {
                buff_special.push(chr);
                if chr != '$' {
                    if chr == '{' {
                        // Remove one of the '$'s,
                        // so "$$${key_" -> "$${key_",
                        // for example
                        buff_special.remove(0);
                        replaced = true;
                    }
                    state = ReplState::Text;
                    buff_out.push_str(&buff_special);
                    buff_special.clear();
                }
            }
            ReplState::Key => {
                if chr == '}' {
                    let repl = replacement(vars, &key, fail_on_missing)?;
                    replaced = replaced || repl.0;
                    buff_out.push_str(&repl.1);
                    key.clear();
                    state = ReplState::Text;
                } else {
                    key.push(chr);
                }
            }
        }
    }

    if replaced {
        buff_out.push_str(&buff_text);
        buff_out.push_str(&buff_special);
        if matches!(state, ReplState::Key) {
            buff_out.push_str("${");
        }
        buff_out.push_str(&key);

        Ok(Cow::Owned(buff_out))
    } else {
        // There was no replacement at all
        // -> return the input
        Ok(Cow::Borrowed(line))
    }
}

/// Replaces all occurences of variables of the form `${KEY}` in a input stream
/// with their respective values.
///
/// # Errors
///
/// If a variable key was found in the stream,
/// but `vars` contains n oentry for it,
/// and `fail_on_missing` is `true`.
///
/// If reading from the input failed.
///
/// If writing to the output failed.
pub fn replace_in_stream(
    vars: &Dict<String>,
    reader: &mut Box<dyn BufRead>,
    writer: &mut Box<dyn Write>,
    fail_on_missing: bool,
) -> io::Result<()> {
    let mut input = String::new();
    // let interval = Duration::from_millis(1);

    loop {
        let n = reader.read_line(&mut input)?;
        if n == 0 {
            // This means most likely that:
            // > This reader has reached its "end of file"
            // > and will likely no longer be able to produce bytes
            // as can be read here:
            // https://docs.w3cub.com/rust/std/io/trait.read#tymethod.read
            //eprintln!("Zero bytes read, ending it here (assuming EOF).");
            break;
        }
        // io::stdout().write_all(repl_vars_in(vars, &input, fail_on_missing)?.as_bytes())?;
        writer.write_all(replace_in_string(vars, &input, fail_on_missing)?.as_bytes())?;

        // thread::sleep(interval);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom:
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_replace_in_string_no_vars() {
        let vars = Dict::<String>::new();
        let input = "a ${key_a} $${key_a} b ${key_b} c";
        let expected = "a ${key_a} ${key_a} b ${key_b} c";
        let actual = replace_in_string(&vars, input, false).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_one_var() {
        let mut vars = Dict::<String>::new();
        vars.add("key_a".to_string(), "1".to_string());
        let input = "a ${key_a} $${key_a} b ${key_b} c";
        let expected = "a 1 ${key_a} b ${key_b} c";
        let actual = replace_in_string(&vars, input, false).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_two_vars() {
        let mut vars = Dict::<String>::new();
        vars.add("key_a".to_string(), "1".to_string());
        vars.add("key_b".to_string(), "2".to_string());
        let input = "a ${key_a} $${key_a} b ${key_b} c";
        let expected = "a 1 ${key_a} b 2 c";
        let actual = replace_in_string(&vars, input, false).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_case_sensitive() {
        let mut vars = Dict::<String>::new();
        vars.add("Key_A".to_string(), "1".to_string());
        vars.add("key_b".to_string(), "2".to_string());
        let input = "a ${key_a} $${key_a} b ${key_b} c";
        let expected = "a ${key_a} ${key_a} b 2 c";
        let actual = replace_in_string(&vars, input, false).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_missing_closing_bracket() {
        let mut vars = Dict::<String>::new();
        vars.add("key_a".to_string(), "1".to_string());
        let input = "a ${key_a";
        let expected = "a ${key_a";
        let actual = replace_in_string(&vars, input, false).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_missing_closing_bracket_and_key() {
        let mut vars = Dict::<String>::new();
        vars.add("key_a".to_string(), "1".to_string());
        let input = "a ${";
        let expected = "a ${";
        let actual = replace_in_string(&vars, input, false).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_missing_closing_bracket_quoted() {
        let mut vars = Dict::<String>::new();
        vars.add("key_a".to_string(), "1".to_string());
        let input = "a $${key_a";
        let expected = "a ${key_a"; // NOTE Do we really want it this way, or should there still be two $$? this way is easy to implement, the other way seems more correct
        let actual = replace_in_string(&vars, input, false).unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_replace_in_string_missing_closing_bracket_and_key_quoted() {
        let vars = Dict::<String>::new();
        let input = "a $${";
        let expected = "a ${"; // NOTE Do we really want it this way, or should there still be two $$? this way is easy to implement, the other way seems more correct
        let actual = replace_in_string(&vars, input, false).unwrap();
        assert_eq!(expected, actual);
    }
}
