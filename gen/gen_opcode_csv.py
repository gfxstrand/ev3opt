#! /usr/bin/python3
#
# Pipe thorugh "column -t -s, -o," to get nice columns

from mako.template import Template
import sys

import parse_enums

TEMPLATE = Template("""\
opcode, subcode, value, proto
% for key, value in enums['Opcode'].items():
${key}, , ${value},
% if key + 'Subcode' in enums:
% for subkey, subval in enums[key + 'Subcode'].items():
${key}, ${subkey}, ${subval},
% endfor
% endif
% endfor
""")

def main():
    try:
        with open(sys.argv[1]) as f:
            print(TEMPLATE.render(enums=parse_enums.parse_enums(f)))

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
