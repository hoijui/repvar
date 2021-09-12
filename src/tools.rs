// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn append_env<S: ::std::hash::BuildHasher>(vars: &mut HashMap<String, String, S>) {
    for env_var in env::vars() {
        vars.insert(env_var.0, env_var.1);
    }
}

#[allow(dead_code)] // This is an API function for lib.rs
pub fn flush_to_env<'a>(vars: impl Iterator<Item = (&'a String, &'a String)>, overwrite: bool) {
    for (key, value) in vars {
        if overwrite || env::var(&key).is_err() {
            env::set_var(&key, &value);
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

/// Creates a reader from a string identifier.
/// Both `None` and `"-"` mean stdin.
///
/// # Errors
///
/// If a file path is specified, and it is not possible to read from it.
pub fn create_input_reader(ident: Option<&str>) -> io::Result<Box<dyn BufRead>> {
    match ident {
        None | Some("-") => Ok(Box::new(BufReader::new(io::stdin()))),
        Some(filename) => {
            let file = File::open(filename)?;
            Ok(Box::new(BufReader::new(file)))
        }
    }
}

/// Creates a writer from a string identifier.
/// Both `None` and `"-"` mean stdout.
///
/// # Errors
///
/// If a file path is specified, and it is not possible to write to it.
pub fn create_output_writer(ident: Option<&str>) -> io::Result<Box<dyn Write>> {
    match ident {
        None | Some("-") => Ok(Box::new(io::stdout()) as Box<dyn Write>),
        Some(file) => {
            let path = Path::new(file);
            let file = File::create(&path)?;
            Ok(Box::new(file) as Box<dyn Write>)
        }
    }
}

pub fn lines_iterator(
    reader: &mut impl BufRead,
) -> impl std::iter::Iterator<Item = io::Result<String>> + '_ {
    // let interval = Duration::from_millis(1);

    let mut buffer = String::new();
    std::iter::from_fn(move || {
        let read_bytes = reader.read_line(&mut buffer);
        match read_bytes {
            Ok(read_bytes) => {
                if read_bytes == 0 {
                    // This means most likely that:
                    // > This reader has reached its "end of file"
                    // > and will likely no longer be able to produce bytes
                    // as can be read here:
                    // https://docs.w3cub.com/rust/std/io/trait.read#tymethod.read
                    //eprintln!("Zero bytes read, ending it here (assuming EOF).");
                    None // end of iterator
                } else {
                    // io::stdout().write_all(repl_vars_in(vars, &buffer, fail_on_missing)?.as_bytes())?;
                    Some(Ok(buffer.clone()))
                }
            }
            Err(err) => Some(Err(err)), // thread::sleep(interval);
        }
    })
}
