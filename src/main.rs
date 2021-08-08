// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{crate_authors, crate_version, App, Arg};
use dict::{Dict, DictIface};
use std::env;
use std::io::Result;

mod key_value;
mod replacer;
mod tools;

fn main() -> Result<()> {
    let args = App::new("repvars")
        .about("Given some text as input, replaces variables of the type `${KEY}` with a respective value.")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::new("input")
                .about("the input file to use; '-' for stdin")
                .takes_value(true)
                .short('i')
                .long("input")
                .multiple_occurrences(false)
                .default_value("-")
                .required(true)
        )
        .arg(
            Arg::new("output")
                .about("the output file to use; '-' for stdout")
                .takes_value(true)
                .short('o')
                .long("output")
                .multiple_occurrences(false)
                .default_value("-")
                .required(true)
        )
        .arg(
            Arg::new("variable")
                .about("a variable key-value pair to be used for substitution in the text")
                .takes_value(true)
                .short('D')
                .long("variable")
                .multiple_occurrences(true)
                .required(false)
        )
        .arg(
            Arg::new("environment")
                .about("use environment variables for substitution in the text")
                .takes_value(false)
                .short('e')
                .long("env")
                .multiple_occurrences(false)
                .required(false)
        )
        .arg(
            Arg::new("verbose")
                .about("more verbose output (useful for debugging)")
                .takes_value(false)
                .short('v')
                .long("verbose")
                .multiple_occurrences(false)
                .required(false)
        )
        .arg(
            Arg::new("fail-on-missing-values")
                .about("fail if no value is available for a variable key found in the input text")
                .takes_value(false)
                .short('f')
                .long("fail-on-missing-values")
                .multiple_occurrences(false)
                .required(false)
        )
        .get_matches();

    let verbose: bool = args.is_present("verbose");

    let mut vars = Dict::<String>::new();

    // enlist environment variables
    if args.is_present("environment") {
        tools::append_env(&mut vars);
    }

    // enlist variables provided on the CLI
    if args.occurrences_of("variable") > 0 {
        for kvp in args
            .values_of_t::<key_value::KeyValuePair>("variable")
            .unwrap_or_else(|e| e.exit())
        {
            vars.add(kvp.key, kvp.value);
        }
    }

    let fail_on_missing: bool = args.is_present("fail-on-missing-values");

    if verbose {
        println!();
        if let Some(ref in_file) = args.value_of("input") {
            println!("INPUT: {}", in_file);
        }
        if let Some(ref out_file) = args.value_of("output") {
            println!("OUTPUT: {}", out_file);
        }

        vars.iter()
            .for_each(|o| println!("VARIABLE: {}={}", o.key, o.val));
        println!();
    }

    let mut reader = tools::create_input_reader(args.value_of("input"))?;
    let mut writer = tools::create_output_writer(args.value_of("output"))?;

    replacer::replace_in_stream(&vars, &mut reader, &mut writer, fail_on_missing)?;

    Ok(())
}
