<!--
SPDX-FileCopyrightText: 2021-2022 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# `repvar` - Variable replacing, UNIX-style text filter

[![License: AGPL-3.0-or-later](
    https://img.shields.io/badge/License-AGPL%203.0+-blue.svg)](
    https://www.gnu.org/licenses/agpl-3.0.html)
[![REUSE status](
    https://api.reuse.software/badge/github.com/hoijui/repvar)](
    https://api.reuse.software/info/github.com/hoijui/repvar)
[![crates.io](
    https://img.shields.io/crates/v/repvar.svg)](
    https://crates.io/crates/repvar)
[![Docs](
    https://docs.rs/repvar/badge.svg)](
    https://docs.rs/repvar)
[![dependency status](
    https://deps.rs/repo/github/hoijui/repvar/status.svg)](
    https://deps.rs/repo/github/hoijui/repvar)
[![Build status](
    https://github.com/hoijui/repvar/workflows/build/badge.svg)](
    https://github.com/hoijui/repvar/actions)

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

### Simplisitc

```bash
$ export KEY_A="replacement"   # setting an env.-variable
$ echo 'Text ${KEY_A}.' \        # input text
    | repvar --env             # replacing variables
Text replacement.              # output
```

### Slightly more elaborate

```bash
$ export first="the environment"
$ echo 'Variables from ${first}, ${second}, ${not_supplied} and $${quoted}.' \
    | repvars --env -D"second=the CLI"
Variables from the environment, the CLI, ${not_supplied} and ${quoted}.
```

More usage info can be seen when running:

```bash
repvars --help
```

## Building

```bash
# To get a binary for your system
cargo build --release

# To get a 64bit binary that is portabel to all Linux systems
scripts/build
```

## Testing

To run unit-, doc- and integration-tests:

```bash
scripts/test
```
