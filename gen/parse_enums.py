#! /usr/bin/python3

import sys
import re

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

    def print(self, values_camel_case):
        # Define the enum
        print('#[allow(dead_code)]')
        print('pub enum ' + self.name() + ' {')
        for key, value in self.items():
            print('    #[allow(non_camel_case_types)]')
            print('    ' + key + ' = ' + value + ',')
        print('}\n')

        # Add some helpers
        print('impl ' + self.name() + ' {')
        print('    #[allow(dead_code)]')
        print('    pub fn from_i32(i: i32) -> std::result::Result<' +
                                    self.name() + ', &\'static str> {')
        print('        match i {')
        for key, value in self.items():
            print('            ' + value + ' => Ok(' + self.name() + '::' + key + '),')
        print('            _ => Err("Invalid enum value for ' + self.name() + '")')
        print('        }')
        print('    }')
        print('')
        print('    #[allow(dead_code)]')
        print('    pub fn to_str(&self) -> &\'static str {')
        print('        match self {')
        for key, value in self.items():
            print('            ' + self.name() + '::' + key + ' => "' + key + '",')
        print('        }')
        print('    }')
        print('}\n')

def parse_enums(f):
    enums = []
    enum = None

    for line in f.readlines():
        if re.match(r'typedef *enum', line):
            assert enum is None
            enum = Enum()
        elif re.match(r'[A-Z_]*;', line):
            enum.set_name(line[0:-2])
            assert enum._values
            enums.append(enum)
            enum = None
        else:
            m = re.match(' *(op)?(?P<name>[0-9A-Z_]+) *= *(?P<value>[x0-9A-F]+).*', line)
            if m:
                enum.add_value(m.group('name'), m.group('value'))

    return enums

with open(sys.argv[1]) as f:
    enums = parse_enums(f)
    for e in enums:
        if e.name == 'OP':
            e.print(True)
        else:
            e.print(False)
