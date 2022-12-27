// SPDX-FileCopyrightText: 2021-2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod cli;
mod key_value;
mod replacer;
mod tools;

use replacer::Settings;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::args_matcher().get_matches();

    let verbose = args.is_present(cli::A_L_VERBOSE);
    let list = args.is_present(cli::A_L_LIST);
    let src = args.value_of(cli::A_L_INPUT);
    let dst = args.value_of(cli::A_L_OUTPUT);

    if list {
        let detected_vars = replacer::extract_from_file(src)?;
        tools::write_to_file(detected_vars, dst)?;
    } else {
        let mut vars = HashMap::new();

        // enlist environment variables
        if args.is_present(cli::A_L_ENVIRONMENT) {
            tools::append_env(&mut vars);
        }
        // enlist variables from files
        if let Some(var_files) = args.values_of(cli::A_L_VARIABLES_FILE) {
            for var_file in var_files {
                let mut reader = tools::create_input_reader(Some(var_file))?;
                vars.extend(key_value::parse_vars_file_reader(&mut reader)?);
            }
        }
        // enlist variables provided on the CLI
        if let Some(variables) = args.values_of(cli::A_L_VARIABLE) {
            for key_value in variables {
                let pair = key_value::Pair::parse(key_value)?;
                vars.insert(pair.key.to_owned(), pair.value.to_owned());
            }
        }

        let fail_on_missing: bool = args.is_present(cli::A_L_FAIL_ON_MISSING_VALUES);

        let settings = settings! {
            vars: vars,
            fail_on_missing: fail_on_missing,
            verbose: verbose
        };

        replacer::replace_in_file(src, dst, &settings)?;
    }

    Ok(())
}
