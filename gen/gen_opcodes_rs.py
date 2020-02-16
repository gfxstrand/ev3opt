#! /usr/bin/python3

LICENSE = """\
/* Copyright © 2020, Jason Ekstrand
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */
"""

from mako.template import Template
import csv
import sys

OPCODE_IDX = 0
SUBCODE_IDX = 1
VALUE_IDX = 2
PROTO_IDX = 3

class Opcode(object):
    def __init__(self, name, value, proto):
        self.name = name
        self.value = value
        self.proto = proto if proto else None
        self.subcodes = None

    def add_subcode(self, subcode):
        if self.subcodes is None:
            self.subcodes = { }
        self.subcodes[subcode.name] = subcode

def parse_csv(f):
    reader = csv.reader(f, delimiter=',')
    header = next(reader)
    assert header[OPCODE_IDX].strip() == 'opcode'
    assert header[SUBCODE_IDX].strip() == 'subcode'
    assert header[VALUE_IDX].strip() == 'value'
    assert header[PROTO_IDX].strip() == 'proto'

    opcodes = { }
    last = None
    for line in reader:
        op = line[OPCODE_IDX].strip()
        subcode = line[SUBCODE_IDX].strip()
        value = line[VALUE_IDX].strip()
        proto = line[PROTO_IDX].strip()
        if subcode:
            last.add_subcode(Opcode(subcode, value, proto))
        else:
            last = Opcode(op, value, proto)
            opcodes[op] = last

    return opcodes

TEMPLATE = Template(LICENSE + """\

/* This file is auto-generated via gen/parse_enums.py. DO NOT EDIT */

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::result::Result;

% for op in opcodes.values():
% if op.subcodes:

#[derive(Copy, Clone)]
pub enum ${op.name}Subcode {
% for subcode in op.subcodes.values():
    ${subcode.name} = ${subcode.value},
% endfor
}

impl ${op.name}Subcode {
    pub fn from_u8(i: u8) -> Result<${op.name}Subcode, &'static str> {
        match i {
% for subcode in op.subcodes.values():
            ${subcode.value} => Ok(${op.name}Subcode::${subcode.name}),
% endfor
            _ => Err("Invalid enum value for ${op.name}Subcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
% for subcode in op.subcodes.values():
            ${op.name}Subcode::${subcode.name} => "${subcode.name}",
% endfor
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}
% endif
% endfor

#[derive(Copy, Clone)]
pub enum Opcode {
% for op in opcodes.values():
% if op.subcodes:
    ${op.name}(${op.name}Subcode),
% else:
    ${op.name},
% endif
% endfor
}

impl Opcode {
    pub fn u8_has_subcode(op: u8) -> bool {
        match op {
% for op in opcodes.values():
% if op.subcodes:
            ${op.value} => true,
% endif
% endfor
            _ => false,
        }
    }

    pub fn from_u8(op: u8, subcode: u8) -> Result<Opcode, &'static str> {
        match op {
% for op in opcodes.values():
% if op.subcodes:
            ${op.value} => Ok(Opcode::${op.name}(${op.name}Subcode::from_u8(subcode)?)),
% else:
            ${op.value} => Ok(Opcode::${op.name}),
% endif
% endfor
            _ => Err("Invalid enum value for Opcode")
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
% for op in opcodes.values():
% if op.subcodes:
            Opcode::${op.name}(val) => match val {
% for subcode in op.subcodes.values():
                ${op.name}Subcode::${subcode.name} => "${op.name}(${subcode.name})",
% endfor
            },
% else:
            Opcode::${op.name} => "${op.name}",
% endif
% endfor
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
% for op in opcodes.values():
% if op.subcodes:
            Opcode::${op.name}(_) => ${op.value},
% else:
            Opcode::${op.name} => ${op.value},
% endif
% endfor
        }
    }
}
""")

def main():
    try:
        with open(sys.argv[1]) as f:
            print(TEMPLATE.render(opcodes=parse_csv(f)))

    except Exception:
        # In the event there's an error, this imports some helpers from mako
        # to print a useful stack trace and prints it, then exits with
        # status 1, if python is run with debug; otherwise it just raises
        # the exception
        if __debug__:
            from mako import exceptions
            sys.stderr.write(exceptions.text_error_template().render() + '\n')
            sys.exit(1)
        raise


if __name__ == '__main__':
    main()
