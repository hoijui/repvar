// SPDX-FileCopyrightText: 2021-2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod cli;
use repvar::key_value;
use repvar::replacer;
use repvar::settings;
use repvar::tools;

use replacer::Settings;
use std::collections::HashMap;

#[allow(clippy::print_stdout)]
fn print_version_and_exit(quiet: bool) {
    #![allow(clippy::print_stdout)]

    if !quiet {
        print!("{} ", clap::crate_name!());
    }
    println!("{}", repvar::VERSION);
    std::process::exit(0);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::args_matcher().get_matches();

    let quiet = args.get_flag(cli::A_L_QUIET);
    let version = args.get_flag(cli::A_L_VERSION);
    if version {
        print_version_and_exit(quiet);
    }

    let verbose = args.get_flag(cli::A_L_VERBOSE);
    let log_level = if verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder().filter_level(log_level).init();
    let list = args.get_flag(cli::A_L_LIST);
    let src = args.get_one::<String>(cli::A_L_INPUT).cloned();
    let dst = args.get_one::<String>(cli::A_L_OUTPUT).cloned();

    if list {
        let detected_vars = replacer::extract_from_file(src.as_deref())?;
        tools::write_to_file(detected_vars, dst.as_deref())?;
    } else {
        let mut vars = HashMap::new();

        // enlist environment variables
        if args.get_flag(cli::A_L_ENVIRONMENT) {
            tools::append_env(&mut vars);
        }
        // enlist variables from files
        if let Some(var_files) = args.get_many::<String>(cli::A_L_VARIABLES_FILE) {
            for var_file in var_files {
                let mut reader = cli_utils::create_input_reader(Some(var_file))?;
                vars.extend(key_value::parse_vars_file_reader(&mut reader)?);
            }
        }
        // enlist variables provided on the CLI
        if let Some(variables) = args.get_many::<String>(cli::A_L_VARIABLE) {
            for key_value in variables {
                let pair = key_value::Pair::parse(key_value)?;
                vars.insert(pair.key.to_owned(), pair.value.to_owned());
            }
        }

        let fail_on_missing = args.get_flag(cli::A_L_FAIL_ON_MISSING_VALUES);

        let settings = settings! {
            vars: vars,
            fail_on_missing: fail_on_missing
        };

        replacer::replace_in_file(src.as_deref(), dst.as_deref(), &settings)?;
    }

    Ok(())
}
