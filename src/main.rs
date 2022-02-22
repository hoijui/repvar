// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{app_from_crate, App, Arg};
use std::collections::HashMap;

mod key_value;
mod tools;
#[macro_use]
mod replacer;

const A_S_INPUT: char = 'i';
const A_L_INPUT: &str = "input";
const A_S_OUTPUT: char = 'o';
const A_L_OUTPUT: &str = "output";
const A_S_VARIABLE: char = 'D';
const A_L_VARIABLE: &str = "variable";
const A_S_ENVIRONMENT: char = 'e';
const A_L_ENVIRONMENT: &str = "env";
const A_S_VERBOSE: char = 'v';
const A_L_VERBOSE: &str = "verbose";
const A_S_FAIL_ON_MISSING_VALUES: char = 'f';
const A_L_FAIL_ON_MISSING_VALUES: &str = "fail-on-missing-values";

fn create_app() -> App<'static> {
    app_from_crate!()
        .about("Given some text as input, replaces variables of the type `${KEY}` with a respective value.")
        .arg(
            Arg::new(A_L_INPUT)
                .help("the input text file to use; '-' for stdin")
                .takes_value(true)
                .short(A_S_INPUT)
                .long(A_L_INPUT)
                .multiple_occurrences(false)
                .default_value("-")
                .required(false)
        )
        .arg(
            Arg::new(A_L_OUTPUT)
                .help("the output text file to use; '-' for stdout")
                .takes_value(true)
                .short(A_S_OUTPUT)
                .long(A_L_OUTPUT)
                .multiple_occurrences(false)
                .default_value("-")
                .required(false)
        )
        .arg(
            Arg::new(A_L_VARIABLE)
                .help("a variable key-value pair to be used for substitution in the text")
                .takes_value(true)
                .short(A_S_VARIABLE)
                .long(A_L_VARIABLE)
                .multiple_occurrences(true)
                .required(false)
        )
        .arg(
            Arg::new(A_L_ENVIRONMENT)
                .help("use environment variables for substitution in the text")
                .takes_value(false)
                .short(A_S_ENVIRONMENT)
                .long(A_L_ENVIRONMENT)
                .multiple_occurrences(false)
                .required(false)
        )
        .arg(
            Arg::new(A_L_VERBOSE)
                .help("more verbose output (useful for debugging)")
                .takes_value(false)
                .short(A_S_VERBOSE)
                .long(A_L_VERBOSE)
                .multiple_occurrences(false)
                .required(false)
        )
        .arg(
            Arg::new(A_L_FAIL_ON_MISSING_VALUES)
                .help("fail if no value is available for a variable key found in the input text")
                .takes_value(false)
                .short(A_S_FAIL_ON_MISSING_VALUES)
                .long(A_L_FAIL_ON_MISSING_VALUES)
                .multiple_occurrences(false)
                .required(false)
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = create_app().get_matches();

    let verbose: bool = args.is_present(A_L_VERBOSE);

    let mut vars = HashMap::new();

    // enlist environment variables
    if args.is_present(A_L_ENVIRONMENT) {
        tools::append_env(&mut vars);
    }
    // enlist variables provided on the CLI
    if let Some(variables) = args.values_of(A_L_VARIABLE) {
        for key_value in variables {
            let pair = key_value::Pair::parse(key_value)?;
            vars.insert(pair.key.to_owned(), pair.value.to_owned());
        }
    }

    let fail_on_missing: bool = args.is_present(A_L_FAIL_ON_MISSING_VALUES);

    let settings = settings! {
        vars: vars,
        fail_on_missing: fail_on_missing,
        verbose: verbose
    };

    let src = args.value_of(A_L_INPUT);
    let dst = args.value_of(A_L_OUTPUT);

    replacer::replace_in_file(src, dst, &settings)?;

    Ok(())
}
