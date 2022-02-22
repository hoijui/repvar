// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;
use thiserror::Error;

use clap::lazy_static::lazy_static;

use crate::tools;

#[derive(Error, Debug)]
#[error("Failed to parse key-value pair; it has to be of the form 'key=value', but was '{input}'")]
pub struct ParseError {
    input: String,
}

impl ParseError {
    fn new(input: &str) -> ParseError {
        ParseError {
            input: input.to_string(),
        }
    }
}

pub struct Pair<'t> {
    pub key: &'t str,
    pub value: &'t str,
}

impl<'t> fmt::Display for Pair<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'='{}'", self.key, self.value)
    }
}

impl<'t> Pair<'t> {
    /// Parses a "KEY=VALUE" string into its two parts.
    ///
    /// # Errors
    ///
    /// If the input string does not contain at least one '=',
    /// and at least one char before and after it.
    pub fn parse(key_value: &'t str) -> std::result::Result<Self, ParseError> {
        let mut splitter = key_value.splitn(2, '=');
        let key = splitter.next().ok_or_else(|| ParseError::new(key_value))?;
        let value = splitter.next().ok_or_else(|| ParseError::new(key_value))?;
        Ok(Pair { key, value })
    }
}

/// Parses a file containing lines string with of the form "KEY=VALUE".
/// Empty lines and those starting with either "#" or "//" are ignored.
///
/// # Errors
///
/// If there is a problem with reading the file.
///
/// If any line has a bad form, missing key and/or value.
/// See [Pair::parse] for more details.
pub fn parse_vars_file_reader(
    mut reader: impl BufRead,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    lazy_static! {
        // Ignore empty lines and those starting with '#' or "//"
        static ref R_IGNORE_LINE: Regex = Regex::new(r"^($|#|//)").unwrap();
    }
    let mut vars = HashMap::<String, String>::new();

    for line in tools::lines_iterator(&mut reader) {
        let line = line?;
        let line = line.trim();
        if !R_IGNORE_LINE.is_match(line) {
            let pair = Pair::parse(line)?;
            let value = tools::unquote(pair.value);
            vars.insert(pair.key.to_owned(), value.to_owned());
        }
    }
    Ok(vars)
}
