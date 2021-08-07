<!--
SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# `repvar` - Variable replacing UNIX-style text filter

[![License: GPL-3.0-or-later](
    https://img.shields.io/badge/License-GPL%203.0+-blue.svg)](
    https://www.gnu.org/licenses/gpl-3.0.html)
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
or be directly supplied through CLI switches
like `-Dkey=value`.

## Usage

A Simplisitc example:

```bash
$ export VAR_A="replacement"   # setting an env.-variable
$ echo 'Text ${VAR}.' \        # input text
    | repvar --env             # filtering with `repvar`
Text replacement.              # output
```

A slightly more elaborate example:

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
cargo build --release
```

## Testing

To run the unit-tests:

```bash
cargo test
```
