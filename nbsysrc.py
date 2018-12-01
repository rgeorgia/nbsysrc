"""NetBSD version of sysrc
This script takes an input that will be appended to  /etc/rc.conf, consiquently it needs to be run as root or sudo.
1. get input from user
2. check if that is already in rc.conf, if not
3. check if the service is in /etc/rc.d, if not
4. check if the service is in /usr/pkg/share/examples/rc.d, if not
5. alert user that the serive is not installed, do you want to add anyway?
5a. Maybe offer to install?
if service not in /etc/rc.d:
    if service not in /usr/pkg/share/examples/rc.d:
        alert user that service is not installed
        answer = add to rc.conf anyway
    else:
        ask user if they want to copy service from example to /etc/rc.d
        if yes:
            copy
if answer == no:
    exit(0)
 else:
    append to  rc.conf
"""
# TODO: verify input is proper rc.conf format.
# TODO: maybe option to insert input at a particular line
import os
import sys
import argparse
import re
from nbrc_meta import RcFactory

rc_fac = RcFactory
rc_data = rc_fac.create(debug_test=True, test_data_dir='data/')


# TODO: need delete line option
def read_args():
    parser = argparse.ArgumentParser(description="Command line to update rc.conf file")
    parser.add_argument('rc_string', nargs='?', metavar='SERVICE=VALUE', help="This is what you want to add")
    parser.add_argument('--list', dest='dest', choices=['etc', 'installed'], help="List available services")
    parser.add_argument('-a', dest='active_services', action='store_true', help="List active services launched from "
                                                                                "/etc/rc.conf")

    return parser.parse_args()


def prt_dir(dir_listing: list):
    maxcolumns = 8
    col_width = 0
    padding = 2

    for y in dir_listing:
        if len(y) > col_width:
            col_width = len(y) + padding

    for cnt, x in enumerate(sorted(dir_listing), start=1):
        if cnt % maxcolumns == 0:
            print()
        else:
            print(f'{x: <{col_width}}', end='')


def main():
    if os.getuid() != 0:
        print(f'You need to be root to run or have sudo access.')
        sys.exit(0)

    rc_file_data = rc_data.read_rc_conf()
    args = read_args()
    etc_rcd_files = rc_data.rc_dot_d_files(rc_data.etc_rc_path)
    example_rcd_files = rc_data.rc_dot_d_files(rc_data.example_rc_path)

    if args.rc_string is not None:
        # rc_data.flags_type = True if '_flags' in args.rc_string else False
        if not bool(re.search(r'\w.*?=\w.*?', args.rc_string)):
            print(f"Invalid input format:\t service=value")
            sys.exit(0)

        service = args.rc_string.split('=')[0]
        result = rc_data.service_in_rc_conf(service=args.rc_string, file_data=rc_file_data)
        # if result.found and bool(re.match(result.current_status, result.desired_status, re.IGNORECASE)):
        if result.found and result.is_same:
            print(f"Service {result.line_value} at line {result.line_number}, doing nothing")
            print("bye")
            sys.exit(0)
        # elif result.found and not bool(re.match(result.current_status, result.desired_status, re.IGNORECASE)):
        elif result.found and not result.is_same:
            answer = input(f"You want to change ({result.line_value}) to ({args.rc_string})? [y/N]~> ")
            if answer == 'y':
                rc_data.replace_line(replacement=args.rc_string, to_replace=result.line_value)

        elif service not in etc_rcd_files and service not in example_rcd_files:
            print(f"Sevice {service} in not in {rc_data.etc_rc_path} or {rc_data.example_rc_path}")
            print(f"You will need to install it first.")
            sys.exit(0)
        elif service not in etc_rcd_files and service in example_rcd_files:
            print(f"{service} is in {rc_data.example_rc_path} but not in {rc_data.etc_rc_path}."
                  f"You will need to:\n")
            print(f"cp {rc_data.etc_rc_path}{service} {rc_data.etc_rc_path}\n")
        else:
            answer = input(f"Add {args.rc_string} to rc.conf? [y/N]~> ").lower()
            if answer == 'y':
                if rc_data.check_input_format(args.rc_string):
                    rc_data.add_line(args.rc_string)
                else:
                    print(f"Bad input format, should take the form of \n\tservice=value {rc_data.enabling_value}")

    if args.dest == 'etc':
        prt_dir(etc_rcd_files)
    elif args.dest == 'installed':
        prt_dir(example_rcd_files)


if __name__ == '__main__':
    main()
