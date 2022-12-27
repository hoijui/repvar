// SPDX-FileCopyrightText: 2021-2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{command, Arg, ArgAction, Command, ValueHint};

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

fn arg_input() -> Arg {
    Arg::new(A_L_INPUT)
        .help("the input text file to use; '-' for stdin")
        .num_args(1)
        .short(A_S_INPUT)
        .long(A_L_INPUT)
        .action(ArgAction::Set)
        .value_hint(ValueHint::FilePath)
        .value_name("FILE")
        .default_value("-")
}

fn arg_output() -> Arg {
    Arg::new(A_L_OUTPUT)
        .help("the output text file to use; '-' for stdout")
        .num_args(1)
        .short(A_S_OUTPUT)
        .long(A_L_OUTPUT)
        .action(ArgAction::Set)
        .value_hint(ValueHint::FilePath)
        .value_name("FILE")
        .default_value("-")
}

fn arg_variable() -> Arg {
    Arg::new(A_L_VARIABLE)
        .help("a variable key-value pair to be used for substitution in the text")
        .num_args(1)
        .short(A_S_VARIABLE)
        .long(A_L_VARIABLE)
        .value_hint(ValueHint::Other)
        .value_name("KEY=VALUE")
        .value_delimiter(',')
        .action(ArgAction::Append)
}

fn arg_variables_file() -> Arg {
    Arg::new(A_L_VARIABLES_FILE)
        .help("An input file containing KEY=VALUE pairs")
        .long_help(
            "An input file containing KEY=VALUE pairs, one per line (BASH style). \
            Empty lines, and those starting with \"#\" or \"//\" are ignored. \
            See -D,--variable for specifying one pair at a time.",
        )
        .num_args(1)
        .value_name("FILE")
        .value_hint(ValueHint::FilePath)
        .short(A_S_VARIABLES_FILE)
        .long(A_L_VARIABLES_FILE)
        .action(ArgAction::Append)
}

fn arg_environment() -> Arg {
    Arg::new(A_L_ENVIRONMENT)
        .help("Use environment variables for substitution in the text")
        .short(A_S_ENVIRONMENT)
        .long(A_L_ENVIRONMENT)
        .action(ArgAction::SetTrue)
}

fn arg_verbose() -> Arg {
    Arg::new(A_L_VERBOSE)
        .help("more verbose output (useful for debugging)")
        .short(A_S_VERBOSE)
        .long(A_L_VERBOSE)
        .action(ArgAction::SetTrue)
}

fn arg_list() -> Arg {
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
        .action(ArgAction::SetTrue)
        .short(A_S_LIST)
        .long(A_L_LIST)
}

fn arg_fail_on_missing_values() -> Arg {
    Arg::new(A_L_FAIL_ON_MISSING_VALUES)
        .help("fail if no value is available for a variable key found in the input text")
        .action(ArgAction::SetTrue)
        .short(A_S_FAIL_ON_MISSING_VALUES)
        .long(A_L_FAIL_ON_MISSING_VALUES)
}

pub fn args_matcher() -> Command {
    command!()
        .about(
            "Given some text as input, \
            replaces variables of the type `${KEY}` with a respective value.",
        )
        .bin_name(clap::crate_name!())
        .help_expected(true)
        .arg(arg_input())
        .arg(arg_output())
        .arg(arg_variable())
        .arg(arg_variables_file())
        .arg(arg_environment())
        .arg(arg_verbose())
        .arg(arg_list())
        .arg(arg_fail_on_missing_values())
}
