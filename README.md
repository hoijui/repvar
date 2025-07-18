<!--
SPDX-FileCopyrightText: 2021-2025 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# `repvar` - Variable replacing, UNIX-style text filter

[![License: AGPL-3.0-or-later](
    https://img.shields.io/badge/License-AGPL%203.0+-blue.svg)](
    LICENSE.txt)
[![REUSE status](
    https://api.reuse.software/badge/github.com/hoijui/repvar)](
    https://api.reuse.software/info/github.com/hoijui/repvar)
[![Repo](
    https://img.shields.io/badge/Repo-GitHub-555555&logo=github.svg)](
    https://github.com/hoijui/repvar)
[![Package Releases](
    https://img.shields.io/crates/v/repvar.svg)](
    https://crates.io/crates/repvar)
[![Documentation Releases](
    https://docs.rs/repvar/badge.svg)](
    https://docs.rs/repvar)
[![Dependency Status](
    https://deps.rs/repo/github/hoijui/repvar/status.svg)](
    https://deps.rs/repo/github/hoijui/repvar)
[![Build Status](
    https://github.com/hoijui/repvar/workflows/build/badge.svg)](
    https://github.com/hoijui/repvar/actions)

[![In cooperation with FabCity Hamburg](
    https://raw.githubusercontent.com/osegermany/tiny-files/master/res/media/img/badge-fchh.svg)](
    https://fabcity.hamburg)
[![In cooperation with Open Source Ecology Germany](
    https://raw.githubusercontent.com/osegermany/tiny-files/master/res/media/img/badge-oseg.svg)](
    https://opensourceecology.de)

A tiny CLI tool that replaces variables of the style `${KEY}`
in text with their respective value.
It can also be used as a rust library.

For the CLI tool,
the variables can be read from the environment
or be directly supplied through CLI switches,
like `-Dkey=value`.

> **NOTE** \
> The author is a rust-newb.
> This crate probably only makes sense for himself,
> and it is not using the power of rust as should be.
> It also could probably be written in just 10 lines of code,
> using one or two regexes, not loosing any performance.

## Usage

### Simplistic

```bash
$ export KEY_A="replacement"   # setting an env.-variable
$ echo 'Text ${KEY_A}.' \      # input text
    | repvar --env             # replacing variables
Text replacement.              # output
```

### Slightly more elaborate

```bash
$ export first="the environment"
$ echo 'Variables from ${first}, ${second}, ${not_supplied} and $${quoted}.' \
    | repvar --env -D"second=the CLI"
Variables from the environment, the CLI, ${not_supplied} and ${quoted}.
```

More usage info can be seen when running:

```bash
repvar --help
```

## Building

```bash
# To get a binary for your system
cargo build --release

# To get a 64bit binary that is portable to all Linux systems
run/rp/build
```

## Testing

To run unit-, doc- and integration-tests:

```bash
run/rp/test
```

## Similar projects

- More powerful string templating engine,
  but very much in line with the unix philosophy as well
  <https://github.com/nilsmartel/string>

## Funding

This project was funded by the European Regional Development Fund (ERDF)
in the context of the [INTERFACER Project](https://www.interfacerproject.eu/),
from August 2021 (project start)
until March 2023.

![Logo of the EU ERDF program](
    https://cloud.fabcity.hamburg/s/TopenKEHkWJ8j5P/download/logo-eu-erdf.png)
