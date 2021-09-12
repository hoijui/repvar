// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use typed_builder::TypedBuilder;

fn replacement<S: ::std::hash::BuildHasher>(
    key: &str,
    settings: &Settings<S>,
) -> io::Result<(bool, String)> {
    return match settings.vars.get(key) {
        Some(val) => Ok((true, val.to_string())),
        None => {
            if settings.fail_on_missing {
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

// #[derive(Debug)]
#[derive(TypedBuilder)]
pub struct Settings<S: ::std::hash::BuildHasher> {
    vars: Box<HashMap<String, String, S>>,
    #[builder(default = false)]
    fail_on_missing: bool,
    #[builder(default = false)]
    verbose: bool,
}

/// Settings builder macro.
///
/// This macro generates builder code,
/// which uses code auto-generated by `TypedBuilder`
/// from the external macro `typed_builder`.
///
/// ```rust
/// # use repvar::replacer::Settings;
/// # use repvar::settings;
/// //#[macro_use(settings)]
/// # use std::collections::HashMap;
/// let mut vars = HashMap::new();
/// // TODO This fails due to some stupid bug(-like thing) regarding Settings not found in this testing environment, even though it is found 2 lines further down
/// //settings! {vars: Box::new(vars)};
/// // expands to:
/// Settings::builder().vars(Box::new(vars)).build();
/// ```
#[macro_export]
macro_rules! settings{
    // match-like arm for the macro
    ($($p:ident:$v:expr),*) => {
        // the macro expands to this code

        // This is always there
        crate::replacer::Settings::builder()
            // This appears as many times as there are arguments
            $(.$p($v))*
            // This too is always there
            .build()
    }
}

/// Replaces all occurences of variables of the form `${KEY}` in a string
/// with their respective values.
///
/// ```rust
/// # use repvar::replacer::{replace_in_string, Settings};
/// # use std::collections::HashMap;
/// let mut vars = HashMap::new();
/// vars.insert("key_a".to_string(), "1".to_string());
/// vars.insert("key_b".to_string(), "2".to_string());
/// let input = "a ${key_a} $${key_a} b ${key_b} c";
/// let expected = "a 1 ${key_a} b 2 c";
/// let actual =
///     replace_in_string(input, &Settings::builder().vars(Box::new(vars)).build()).unwrap();
/// assert_eq!(expected, actual);
/// ```
///
/// # Errors
///
/// If a variable key was found in the stream,
/// but `vars` contains no entry for it,
/// and `fail_on_missing` is `true`.
pub fn replace_in_string<'t, S: ::std::hash::BuildHasher>(
    line: &'t str,
    settings: &Settings<S>,
) -> io::Result<Cow<'t, str>> {
    let mut state = ReplState::Text;
    let mut key = String::with_capacity(64);
    let mut buff_special = String::with_capacity(5);
    let mut buff_out = String::with_capacity(line.len() * 3 / 2);
    let mut replaced = false;
    for chr in line.chars() {
        match state {
            ReplState::Text => {
                if chr == '$' {
                    state = ReplState::Dollar1;
                    buff_special.push(chr);
                } else {
                    buff_out.push(chr);
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
                    let repl = replacement(&key, settings)?;
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
/// but `vars` contains no entry for it,
/// and `fail_on_missing` is `true`.
///
/// If reading from the `reader` failed.
///
/// If writing to the `writer` failed.
pub fn replace_in_stream<S: ::std::hash::BuildHasher>(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
    settings: &Settings<S>,
) -> io::Result<()> {
    if settings.verbose {
        println!();
        for (key, value) in &*settings.vars {
            println!("VARIABLE: {}={}", key, value);
        }
        println!();
    }

    for line in crate::tools::lines_iterator(reader) {
        writer.write_all(replace_in_string(&line?, settings)?.as_bytes())?;
    }

    Ok(())
}

/// Replaces all occurences of variables of the form `${KEY}` in a input stream
/// with their respective values.
///
/// # Errors
///
/// If a variable key was found in the stream,
/// but `vars` contains no entry for it,
/// and `fail_on_missing` is `true`.
///
/// If reading from the `source` failed.
///
/// If writing to the `destination` failed.
pub fn replace_in_file<S: ::std::hash::BuildHasher>(
    source: Option<&str>,
    destination: Option<&str>,
    settings: &Settings<S>,
) -> io::Result<()> {
    if settings.verbose {
        println!();
        if let Some(in_file) = source {
            println!("INPUT: {}", &in_file);
        }
        if let Some(out_file) = destination {
            println!("OUTPUT: {}", &out_file);
        }
        println!();
    }

    let mut reader = crate::tools::create_input_reader(source)?;
    let mut writer = crate::tools::create_output_writer(destination)?;

    replace_in_stream(&mut reader, &mut writer, settings)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom:
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_replace_in_string_no_vars() {
        let vars = HashMap::new();
        let input = "a ${key_a} $${key_a} b ${key_b} c";
        let expected = "a ${key_a} ${key_a} b ${key_b} c";
        let actual = replace_in_string(input, &settings! {vars: Box::new(vars)}).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_one_var() {
        let mut vars = HashMap::new();
        vars.insert("key_a".to_string(), "1".to_string());
        let input = "a ${key_a} $${key_a} b ${key_b} c";
        let expected = "a 1 ${key_a} b ${key_b} c";
        let actual = replace_in_string(input, &settings! {vars: Box::new(vars)}).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_two_vars() {
        let mut vars = HashMap::new();
        vars.insert("key_a".to_string(), "1".to_string());
        vars.insert("key_b".to_string(), "2".to_string());
        let input = "a ${key_a} $${key_a} b ${key_b} c";
        let expected = "a 1 ${key_a} b 2 c";
        let actual = replace_in_string(input, &settings! {vars: Box::new(vars)}).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_case_sensitive() {
        let mut vars = HashMap::new();
        vars.insert("Key_A".to_string(), "1".to_string());
        vars.insert("key_b".to_string(), "2".to_string());
        let input = "a ${key_a} $${key_a} b ${key_b} c";
        let expected = "a ${key_a} ${key_a} b 2 c";
        let actual = replace_in_string(input, &settings! {vars: Box::new(vars)}).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_missing_closing_bracket() {
        let mut vars = HashMap::new();
        vars.insert("key_a".to_string(), "1".to_string());
        let input = "a ${key_a";
        let expected = "a ${key_a";
        let actual = replace_in_string(input, &settings! {vars: Box::new(vars)}).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_missing_closing_bracket_and_key() {
        let mut vars = HashMap::new();
        vars.insert("key_a".to_string(), "1".to_string());
        let input = "a ${";
        let expected = "a ${";
        let actual = replace_in_string(input, &settings! {vars: Box::new(vars)}).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_replace_in_string_missing_closing_bracket_quoted() {
        let mut vars = HashMap::new();
        vars.insert("key_a".to_string(), "1".to_string());
        let input = "a $${key_a";
        let expected = "a ${key_a"; // NOTE Do we really want it this way, or should there still be two $$? this way is easy to implement, the other way seems more correct
        let actual = replace_in_string(input, &settings! {vars: Box::new(vars)}).unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_replace_in_string_missing_closing_bracket_and_key_quoted() {
        let vars = HashMap::new();
        let input = "a $${";
        let expected = "a ${"; // NOTE Do we really want it this way, or should there still be two $$? this way is easy to implement, the other way seems more correct
        let actual = replace_in_string(input, &settings! {vars: Box::new(vars)}).unwrap();
        assert_eq!(expected, actual);
    }
}
