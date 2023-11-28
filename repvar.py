# SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

'''
DEPRECATED: This is outdated, please use the rust version of repvar!

This was written before the rust version.
It:
* has much less features,
* is more buggy,
* has no unit-tests and
* runs much slower.

Utilities for parsing text that contains variables
of the kind `${KEY}`, and replacing those with actual values.
There is also code for parsing text consisting of key-value pairs,
separated either by `=` or `:`.
'''

import os
import sys
from enum import Enum

def repl_key(key: str, fail: False) -> (bool, str):
    ok = True
    try:
        val = os.environ[key]
    except:
        print('WARNING: No value (environment variable) supplied for key "%s"' % key, file=sys.stderr)
        ok = not fail
        val = '${%s}' % key
    return (ok, val)

class ReplState(Enum):
    TEXT = 1
    DOLLAR1 = 2
    DOLLAR2 = 3
    KEY = 6

def repl_vars(line, fail: bool) -> (bool, str):
    state = ReplState.TEXT
    key = ''
    buff_text = ''
    buff_special = ''
    buff_out = ''
    ok = False
    for chr in line:
        if state == ReplState.TEXT:
            if chr == '$':
                state = ReplState.DOLLAR1
                buff_out = buff_out + buff_text
                buff_text = ''
                buff_special = buff_special + chr
            else:
                buff_text = buff_text + chr
        elif state == ReplState.DOLLAR1:
            if chr == '$':
                state = ReplState.DOLLAR2
                buff_special = buff_special + chr
            elif chr == '{':
                state = ReplState.KEY
                buff_special = ''
            else:
                state = ReplState.TEXT
                buff_out = buff_out + buff_special
                buff_special = ''
        elif state == ReplState.DOLLAR2:
            buff_special = buff_special + chr
            if chr != '$':
                state == ReplState.TEXT
                buff_out = buff_out + buff_special
                buff_special = ''
        elif state == ReplState.KEY:
            if chr == '}':
                (ok, val) = repl_key(key, fail)
                if not ok:
                    break
                buff_out = buff_out + val
                key = ''
                state = ReplState.TEXT
            else:
                key = key + chr
    return (ok, buff_out)

def repvar():
    fail = False

    ok = True
    for line in sys.stdin:
        (ok, line) = repl_vars(line, fail)
        if not ok:
            break
        print(line)
    if not ok:
        sys.exit(1)


if __name__ == '__main__':
    print('DEPRECATED: This is outdated, please use the rust version of repvar!', file=sys.stderr)
    repvar()
