// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod key_value;
pub mod replacer;
pub mod tools;

use git_version::git_version;

// This tests rust code in the README with doc-tests.
// Though, It will not appear in the generated documentaton.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub const VERSION: &str = git_version!();
