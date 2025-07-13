// SPDX-FileCopyrightText: 2022-2024 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use cli_utils::BoxResult;
use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Failed to parse key-value pair; it has to be of the form 'key=value', but was '{input}'")]
pub struct ParseError {
    input: String,
}

impl ParseError {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }
}

/// Owned version of [`Pair`],
/// used when importing this crate as a library,
/// e.g. for parsing cli args into pairs with clap.
#[derive(Clone)]
pub struct PairBuf {
    pub key: String,
    pub value: String,
}

impl PairBuf {
    /// Parses a "KEY=VALUE" string into its two parts.
    ///
    /// # Errors
    ///
    /// If the input string does not contain at least one '=',
    /// and at least one char before and after it.
    pub fn parse(key_value: &str) -> std::result::Result<Self, ParseError> {
        Ok(Pair::parse(key_value)?.to_pair_buf())
    }
}

pub struct Pair<'t> {
    pub key: &'t str,
    pub value: &'t str,
}

impl fmt::Display for Pair<'_> {
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

    /// Converts a `Pair` into a `PairBuf`.
    ///
    /// This makes sense if you need to store the parsed key-value
    /// pair in a data-structure that requires owned strings.
    #[must_use]
    pub fn to_pair_buf(&self) -> PairBuf {
        PairBuf {
            key: self.key.to_owned(),
            value: self.value.to_owned(),
        }
    }
}

/// Parses a file containing lines of the form `KEY=VALUE`.
///
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
pub fn parse_vars_file_reader(reader: impl BufRead) -> BoxResult<HashMap<String, String>> {
    let iter = dotenvy::Iter::new(reader);
    let vars: Result<HashMap<_, _>, _> = iter.into_iter().collect();
    Ok(vars?)
}
