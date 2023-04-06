// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

pub fn append_env<S: ::std::hash::BuildHasher>(vars: &mut HashMap<String, String, S>) {
    for env_var in env::vars() {
        vars.insert(env_var.0, env_var.1);
    }
}

#[allow(dead_code)] // This is an API function for lib.rs
pub fn flush_to_env<'a>(vars: impl Iterator<Item = (&'a String, &'a String)>, overwrite: bool) {
    for (key, value) in vars {
        if overwrite || env::var(key).is_err() {
            env::set_var(key, value);
        }
    }
}

#[allow(dead_code)] // This is an API function for lib.rs
pub fn flush_map_to_env<S: ::std::hash::BuildHasher>(
    vars: &HashMap<String, String, S>,
    overwrite: bool,
) {
    flush_to_env(
        Box::new(
            vars.iter()
                .map(|key_and_value| (key_and_value.0, key_and_value.1)),
        ),
        overwrite,
    );
}

/// Returns the quoting character,
/// if the supplied string starts with one,
/// else returns `None`.
///
/// ```
/// # use repvar::tools::get_start_quote;
/// assert_eq!(get_start_quote(r#""Hello World""#), Some('"'));
/// assert_eq!(get_start_quote(r#" "Hello World" "#), None);
/// assert_eq!(get_start_quote(r#" "Hello World""#), None);
/// assert_eq!(get_start_quote(r#""Hello World" "#), Some('"'));
/// assert_eq!(get_start_quote(r#""Hello World"#), Some('"'));
/// assert_eq!(get_start_quote(r#"Hello World""#), None);
/// assert_eq!(get_start_quote(r#"'Hello World'"#), Some('\''));
/// assert_eq!(get_start_quote(r#" 'Hello World' "#), None);
/// assert_eq!(get_start_quote(r#" 'Hello World'"#), None);
/// assert_eq!(get_start_quote(r#"'Hello World' "#), Some('\''));
/// assert_eq!(get_start_quote(r#"'Hello World"#), Some('\''));
/// assert_eq!(get_start_quote(r#"Hello World'"#), None);
/// assert_eq!(get_start_quote(r#"Hello World"#), None);
/// ```
#[must_use]
pub fn get_start_quote(pot_quoted: &str) -> Option<char> {
    if let Some(first) = pot_quoted.chars().next() {
        if first == '"' || first == '\'' {
            return Some(first);
        }
    }
    None
}

/// Returns the quoting character,
/// if the supplied string ends with one,
/// else returns `None`.
///
/// ```
/// # use repvar::tools::get_end_quote;
/// assert_eq!(get_end_quote(r#""Hello World""#), Some('"'));
/// assert_eq!(get_end_quote(r#" "Hello World" "#), None);
/// assert_eq!(get_end_quote(r#" "Hello World""#), Some('"'));
/// assert_eq!(get_end_quote(r#""Hello World" "#), None);
/// assert_eq!(get_end_quote(r#""Hello World"#), None);
/// assert_eq!(get_end_quote(r#"Hello World""#), Some('"'));
/// assert_eq!(get_end_quote(r#"'Hello World'"#), Some('\''));
/// assert_eq!(get_end_quote(r#" 'Hello World' "#), None);
/// assert_eq!(get_end_quote(r#" 'Hello World'"#), Some('\''));
/// assert_eq!(get_end_quote(r#"'Hello World' "#), None);
/// assert_eq!(get_end_quote(r#"'Hello World"#), None);
/// assert_eq!(get_end_quote(r#"Hello World'"#), Some('\''));
/// assert_eq!(get_end_quote(r#"Hello World"#), None);
/// ```
#[must_use]
pub fn get_end_quote(pot_quoted: &str) -> Option<char> {
    if let Some(last) = pot_quoted.chars().last() {
        if last == '"' || last == '\'' {
            return Some(last);
        }
    }
    None
}

/// Returns an unquoted version of the input string,
/// or the input string its self,
/// if it was not quoted in the first place.
///
/// ```
/// # use repvar::tools::unquote;
/// assert_eq!(unquote(r#""Hello World""#), r#"Hello World"#);
/// assert_eq!(unquote(r#" "Hello World" "#), r#" "Hello World" "#);
/// assert_eq!(unquote(r#" "Hello World""#), r#" "Hello World""#);
/// assert_eq!(unquote(r#""Hello World" "#), r#""Hello World" "#);
/// assert_eq!(unquote(r#""Hello World"#), r#""Hello World"#);
/// assert_eq!(unquote(r#"Hello World""#), r#"Hello World""#);
/// assert_eq!(unquote(r#"'Hello World'"#), r#"Hello World"#);
/// assert_eq!(unquote(r#" 'Hello World' "#), r#" 'Hello World' "#);
/// assert_eq!(unquote(r#" 'Hello World'"#), r#" 'Hello World'"#);
/// assert_eq!(unquote(r#"'Hello World' "#), r#"'Hello World' "#);
/// assert_eq!(unquote(r#"'Hello World"#), r#"'Hello World"#);
/// assert_eq!(unquote(r#"Hello World'"#), r#"Hello World'"#);
/// assert_eq!(unquote(r#"Hello World"#), r#"Hello World"#);
/// assert_eq!(unquote(r#""Hello World'"#), r#""Hello World'"#);
/// assert_eq!(unquote(r#"'Hello World""#), r#"'Hello World""#);
/// ```
#[must_use]
pub fn unquote(pot_quoted: &str) -> &str {
    let len = pot_quoted.len();
    if len > 1 {
        if let (Some(start_q), Some(end_q)) =
            (get_start_quote(pot_quoted), get_end_quote(pot_quoted))
        {
            if start_q == end_q {
                return &pot_quoted[1..len - 1];
            }
        }
    }
    pot_quoted
}

/// Writes a list of strings to a file.
///
/// # Errors
///
/// If writing to `destination` failed.
pub fn write_to_file(lines: Vec<String>, destination: Option<&str>) -> io::Result<()> {
    let mut writer = cli_utils::create_output_writer(destination)?;

    for line in lines {
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
