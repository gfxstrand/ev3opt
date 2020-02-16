#! /usr/bin/python3

from mako.template import Template
import re
import sys

RENAMES = {
    'OP'            : 'Opcode',
    'HWTYPE'        : 'HWType',
    'NXTCOLOR'      : 'NXTColor',
    'BLACKCOLOR'    : 'BLACK',
    'BLUECOLOR'     : 'BLUE',
    'GREENCOLOR'    : 'GREEN',
    'YELLOWCOLOR'   : 'YELLOW',
    'REDCOLOR'      : 'RED',
    'WHITECOLOR'    : 'WHITE',
    'ADDF'          : 'AddF',
    'SUBF'          : 'SubF',
    'MULF'          : 'MulF',
    'DIVF'          : 'DivF',
    'MOVEF_8'       : 'MoveF_8',
    'MOVEF_16'      : 'MoveF_16',
    'MOVEF_32'      : 'MoveF_32',
    'MOVEF_F'       : 'MoveF_F',
    'MATHTYPE'      : 'MathSubcode',
    'STRINGS'       : 'String',
    'INPUT_READSI'  : 'InputReadSi',
    'INPUT_READEXT' : 'InputReadExt',
}

def to_camel_case(s):
    words = []
    upper = True
    for w in s.split('_'):
        # Leave numbers separated by _
        if words and words[-1][-1].isnumeric():
            words.append('_')
        words.append(w.lower().capitalize())
    return ''.join(words)

def sanitize_name(name, camel_case):
    if name != 'TST_SUBCODE' and name.startswith('TST_'):
        name = name[4:]

    if name in RENAMES:
        return RENAMES[name]
    elif camel_case:
        return to_camel_case(name)
    else:
        return name

class Enum(object):
    def __init__(self):
        self._name = None
        self._values = { }

    def set_name(self, name):
        self._name = name

    def add_value(self, key, value):
        self._values[key] = value

    def name(self):
        return sanitize_name(self._name, True)

    def items(self):
        for key, value in self._values.items():
            yield sanitize_name(key, self._name == 'OP'), value

def parse_enums(f):
    enums = { }
    enum = None

    for line in f.readlines():
        if re.match(r'typedef *enum', line):
            assert enum is None
            enum = Enum()
        elif re.match(r'[A-Z_]*;', line):
            enum.set_name(line[0:-2])
            assert enum._values
            enums[enum.name()] = enum
            enum = None
        else:
            m = re.match(' *(op)?(?P<name>[0-9A-Z_]+) *= *(?P<value>[x0-9A-F]+).*', line)
            if m:
                enum.add_value(m.group('name'), m.group('value'))

    return enums

TEMPLATE = Template("""\
% for enum in enums.values():
#[allow(dead_code)]
pub enum ${enum.name()} {
% for key, value in enum.items():
    #[allow(non_camel_case_types)]
    ${key} = ${value},
% endfor
}

impl ${enum.name()} {
    #[allow(dead_code)]
    pub fn from_i32(i: i32) -> std::result::Result<${enum.name()}, &'static str> {
        match i {
% for key, value in enum.items():
            ${value} => Ok(${enum.name()}::${key}),
% endfor
            _ => Err("Invalid enum value for ${enum.name()}")
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
% for key, value in enum.items():
            ${enum.name()}::${key} => "${key}",
% endfor
        }
    }
}
% endfor
""")

with open(sys.argv[1]) as f:
    try:
        print(TEMPLATE.render(enums=parse_enums(f)))
    except Exception:
        # In the event there's an error, this imports some helpers from mako
        # to print a useful stack trace and prints it, then exits with
        # status 1, if python is run with debug; otherwise it just raises
        # the exception
        if __debug__:
            import sys
            from mako import exceptions
            sys.stderr.write(exceptions.text_error_template().render() + '\n')
            sys.exit(1)
        raise
