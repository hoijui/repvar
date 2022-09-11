// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;
use thiserror::Error;

use lazy_static::lazy_static;

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

/// Parses a file containing lines of the form `KEY=VALUE`.
/// Empty lines and those starting with either "#" or "//" are ignored.
/// Values may be quoted: `KEY="VALUE"` or `KEY='VALUE'`.
/// Multi-line values are possible too; they require quotes:
/// `KEY="A value made up
/// of
/// multiple lines"`
///
/// # Errors
///
/// If there is a problem with reading the file.
///
/// If any line has a bad form, missing key and/or value.
/// See [``Pair::parse``] for more details.
pub fn parse_vars_file_reader(
    reader: impl BufRead,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    lazy_static! {
        // Ignore empty lines and those starting with '#' or "//"
        static ref R_IGNORE_LINE: Regex = Regex::new(r"^($|#|//)").unwrap();
    }
    let iter = dotenv::iter::Iter::new(reader);
    let vars: Result<HashMap<_, _>, _> = iter.into_iter().collect();
    Ok(vars?)
}
