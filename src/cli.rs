// SPDX-FileCopyrightText: 2021-2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{app_from_crate, App, Arg, ValueHint};

pub const A_S_INPUT: char = 'i';
pub const A_L_INPUT: &str = "input";
pub const A_S_OUTPUT: char = 'o';
pub const A_L_OUTPUT: &str = "output";
pub const A_S_VARIABLE: char = 'D';
pub const A_L_VARIABLE: &str = "variable";
pub const A_S_VARIABLES_FILE: char = 'I';
pub const A_L_VARIABLES_FILE: &str = "variables-file";
pub const A_S_ENVIRONMENT: char = 'e';
pub const A_L_ENVIRONMENT: &str = "env";
pub const A_S_VERBOSE: char = 'v';
pub const A_L_VERBOSE: &str = "verbose";
pub const A_S_LIST: char = 'l';
pub const A_L_LIST: &str = "list";
pub const A_S_FAIL_ON_MISSING_VALUES: char = 'f';
pub const A_L_FAIL_ON_MISSING_VALUES: &str = "fail-on-missing-values";

pub fn args_matcher() -> App<'static> {
    app_from_crate!()
        .about(
            "Given some text as input, \
            replaces variables of the type `${KEY}` with a respective value.",
        )
        .arg(
            Arg::new(A_L_INPUT)
                .help("the input text file to use; '-' for stdin")
                .takes_value(true)
                .short(A_S_INPUT)
                .long(A_L_INPUT)
                .multiple_occurrences(false)
                .default_value("-")
                .required(false),
        )
        .arg(
            Arg::new(A_L_OUTPUT)
                .help("the output text file to use; '-' for stdout")
                .takes_value(true)
                .short(A_S_OUTPUT)
                .long(A_L_OUTPUT)
                .multiple_occurrences(false)
                .default_value("-")
                .required(false),
        )
        .arg(
            Arg::new(A_L_VARIABLE)
                .help("a variable key-value pair to be used for substitution in the text")
                .takes_value(true)
                .short(A_S_VARIABLE)
                .long(A_L_VARIABLE)
                .multiple_occurrences(true)
                .required(false),
        )
        .arg(
            Arg::new(A_L_VARIABLES_FILE)
                .help("An input file containing KEY=VALUE pairs")
                .long_help(
                    "An input file containing KEY=VALUE pairs, one per line (BASH style). \
                    Empty lines, and those starting with \"#\" or \"//\" are ignored. \
                    See -D,--variable for specifying one pair at a time.",
                )
                .takes_value(true)
                .forbid_empty_values(true)
                .value_name("FILE")
                .value_hint(ValueHint::FilePath)
                .short(A_S_VARIABLES_FILE)
                .long(A_L_VARIABLES_FILE)
                .multiple_occurrences(true)
                .required(false),
        )
        .arg(
            Arg::new(A_L_ENVIRONMENT)
                .help("use environment variables for substitution in the text")
                .takes_value(false)
                .short(A_S_ENVIRONMENT)
                .long(A_L_ENVIRONMENT)
                .multiple_occurrences(false)
                .required(false),
        )
        .arg(
            Arg::new(A_L_VERBOSE)
                .help("more verbose output (useful for debugging)")
                .takes_value(false)
                .short(A_S_VERBOSE)
                .long(A_L_VERBOSE)
                .multiple_occurrences(false)
                .required(false),
        )
        .arg(
            Arg::new(A_L_LIST)
                .help("Only list the variables found in the input text, and exit")
                .long_help(
                    "Only list the variables found in the input text in the output, \
                    instead of the input text with the variables replaced. \
                    The variables will appear in the output in the same order as in the input, \
                    one per line, \
                    and as many time as they appear in the input; \
                    i.e. there will be duplicates.",
                )
                .takes_value(false)
                .short(A_S_LIST)
                .long(A_L_LIST)
                .multiple_occurrences(false)
                .required(false),
        )
        .arg(
            Arg::new(A_L_FAIL_ON_MISSING_VALUES)
                .help("fail if no value is available for a variable key found in the input text")
                .takes_value(false)
                .short(A_S_FAIL_ON_MISSING_VALUES)
                .long(A_L_FAIL_ON_MISSING_VALUES)
                .multiple_occurrences(false)
                .required(false),
        )
}
