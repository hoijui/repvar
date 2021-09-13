// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use assert_cmd::{prelude::*, Command}; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::{
    collections::{hash_map::RandomState, HashMap},
    fs,
    path::Path,
};
use tempfile::NamedTempFile;

pub struct RepVar<'a, S = RandomState> {
    cwd: Option<&'a str>,
    stdin: Option<&'a str>,
    env_vars: HashMap<&'a str, &'a str, S>,
    args: Vec<&'a str>,
    stdout: Option<&'a str>,
    stderr: Option<&'a str>,
}

impl<'a> RepVar<'a> {
    pub fn new() -> RepVar<'a> {
        RepVar {
            cwd: None,
            stdin: None,
            env_vars: HashMap::new(),
            args: Vec::new(),
            stdout: None,
            stderr: None,
        }
    }

    /// Set the working directory for the child process.
    pub fn cwd(&'a mut self, dir: &'a str) -> &'a mut RepVar {
        self.cwd = Some(dir);
        self
    }

    pub fn stdin(&'a mut self, text: &'a str) -> &'a mut RepVar {
        self.stdin = Some(text);
        self
    }

    /// Add an argument to pass to the program.
    pub fn env(&'a mut self, key: &'a str, value: &'a str) -> &'a mut RepVar {
        self.env_vars.insert(key, value);
        self
    }

    /// Add an argument to pass to the program.
    pub fn arg(&'a mut self, arg: &'a str) -> &'a mut RepVar {
        self.args.push(arg);
        self
    }

    /// Add multiple arguments to pass to the program.
    pub fn args(&'a mut self, args: &[&'a str]) -> &'a mut RepVar {
        self.args.extend_from_slice(args);
        self
    }

    pub fn stdout(&'a mut self, text: &'a str) -> &'a mut RepVar {
        self.stdout = Some(text);
        self
    }

    pub fn stderr(&'a mut self, text: &'a str) -> &'a mut RepVar {
        self.stderr = Some(text);
        self
    }

    fn run_test(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("repvar")?;
        cmd.env_clear();

        // Prepares the command
        for (key, value) in &self.env_vars {
            cmd.env(key, value);
        }
        for arg in &self.args {
            cmd.arg(arg);
        }
        if let Some(stdin) = self.stdin {
            cmd.write_stdin(stdin);
        }

        // Runs the command
        let mut assert = cmd.assert();

        // Evaluates the command
        assert = if let Some(stdout) = self.stdout {
            assert.stdout(predicate::eq(stdout))
        } else {
            assert.failure()
        };
        if let Some(stderr) = self.stderr {
            assert.stderr(predicate::str::contains(stderr));
        }

        Ok(())
    }
}

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .arg("-DKEY=value")
        .stdin("This text contains a ${KEY} in its middle.")
        .stdout("This text contains a value in its middle.")
        .run_test()
}

#[test]
fn simple_space() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .arg("-D")
        .arg("KEY=value")
        .stdin("This text contains a ${KEY} in its middle.")
        .stdout("This text contains a value in its middle.")
        .run_test()
}

#[test]
fn simple_equals() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .arg("-D=KEY=value")
        .stdin("This text contains a ${KEY} in its middle.")
        .stdout("This text contains a value in its middle.")
        .run_test()
}

#[test]
fn simple_long() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .arg("--variable")
        .arg("KEY=value")
        .stdin("This text contains a ${KEY} in its middle.")
        .stdout("This text contains a value in its middle.")
        .run_test()
}

#[test]
fn simple_long_equals() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .arg("--variable=KEY=value")
        .stdin("This text contains a ${KEY} in its middle.")
        .stdout("This text contains a value in its middle.")
        .run_test()
}

#[test]
fn quoting() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .arg("-DKEY=value")
        .stdin("This text contains a quoted $${KEY}.")
        .stdout("This text contains a quoted ${KEY}.")
        .run_test()
}

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .arg("--input")
        .arg("test/file/does/not/exist")
        .stderr("No such file or directory")
        .run_test()
}

fn write_to_file(file: &Path, text: &str) {
    fs::write(file, text).expect("Unable to write file");
}

#[test]
fn file_does_exist() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    write_to_file(file.path(), "This text contains a ${KEY} in its middle.\n");
    let file_path_string = file.path().as_os_str().to_str().ok_or("Non UTF-8 string")?;

    RepVar::new()
        .arg("-DKEY=value")
        .arg("-i")
        .arg(file_path_string)
        .stdout("This text contains a value in its middle.\n")
        .run_test()
}

#[test]
fn file_does_exist_long() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    write_to_file(file.path(), "This text contains a ${KEY} in its middle.\n");
    let file_path_string = file.path().as_os_str().to_str().ok_or("Non UTF-8 string")?;

    RepVar::new()
        .arg("-DKEY=value")
        .arg("--input")
        .arg(file_path_string)
        .stdout("This text contains a value in its middle.\n")
        .run_test()
}

#[test]
fn fail_not_on_missing_values() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .stdin("This text contains a ${KEY} in its middle.")
        .stdout("This text contains a ${KEY} in its middle.")
        .run_test()
}

#[test]
fn fail_on_missing_values() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .arg("-f")
        .stdin("This text contains a ${KEY} in its middle.")
        .stderr("Undefined variable 'KEY'")
        .run_test()
}

#[test]
fn fail_on_missing_values_long() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .arg("--fail-on-missing-values")
        .stdin("This text contains a ${KEY} in its middle.")
        .stderr("Undefined variable 'KEY'")
        .run_test()
}

#[test]
fn env() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .env("KEY", "value")
        .arg("-e")
        .stdin("This text contains a ${KEY} in its middle.")
        .stdout("This text contains a value in its middle.")
        .run_test()
}

#[test]
fn env_long() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .env("KEY", "value")
        .arg("--env")
        .stdin("This text contains a ${KEY} in its middle.")
        .stdout("This text contains a value in its middle.")
        .run_test()
}

#[test]
fn env_disabled() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .env("KEY", "value")
        .stdin("This text contains a ${KEY} in its middle.")
        .stdout("This text contains a ${KEY} in its middle.")
        .run_test()
}

#[test]
fn env_disabled_fail() -> Result<(), Box<dyn std::error::Error>> {
    RepVar::new()
        .env("KEY", "value")
        .arg("--fail-on-missing-values")
        .stdin("This text contains a ${KEY} in its middle.")
        .stderr("Undefined variable 'KEY'")
        .run_test()
}
