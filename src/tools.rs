// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use dict::{Dict, DictIface};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn append_env(vars: &mut Dict<String>) {
    for env_var in env::vars() {
        vars.add(env_var.0, env_var.1);
    }
}

pub fn create_input_reader(ident: Option<&str>) -> Box<dyn BufRead> {
    match ident {
        None | Some("-") => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    }
}

pub fn create_output_writer(ident: Option<&str>) -> Box<dyn Write> {
    match ident {
        None | Some("-") => Box::new(io::stdout()) as Box<dyn Write>,
        Some(file) => {
            let path = Path::new(file);
            Box::new(File::create(&path).unwrap()) as Box<dyn Write>
        }
    }
}
