// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use regex::Regex;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct KeyValueParseError {
    details: String,
}

impl KeyValueParseError {
    fn new(msg: &str) -> KeyValueParseError {
        KeyValueParseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for KeyValueParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for KeyValueParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

impl fmt::Display for KeyValuePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'='{}'", self.key, self.value)
    }
}

impl FromStr for KeyValuePair {
    type Err = KeyValueParseError;

    fn from_str(blob: &str) -> std::result::Result<Self, Self::Err> {
        let valid: Regex = Regex::new(r"^[^=\\0]+=[^=\\0]+$").unwrap();
        if valid.is_match(blob) {
            let mut parts = blob.split('=');
            Ok(KeyValuePair {
                key: parts.next().unwrap().to_string(),
                value: parts.next().unwrap().to_string(),
            })
        } else {
            Err(KeyValueParseError::new(
                "Invalid key-value pair; exactly and only one '=' and no '\\0' is allowed",
            ))
        }
    }
}
