#!/usr/bin/env python3.7
"""NetBSD version of sysrc
This script takes an input that will be appended to  /etc/rc.conf, consequently it needs to be run
as root or sudo.
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

import sys
import argparse
import re
from nbrc_meta import NetBsdRc


# TODO: need delete line option
def read_args():
    """Read and process command line args"""
    parser = argparse.ArgumentParser(description="Command line to update rc.conf file")
    parser.add_argument('rc_string', nargs='?', metavar='SERVICE=VALUE',
                        help="This is what you want to add")
    list_group = parser.add_mutually_exclusive_group()
    # parser.add_argument('--list', dest='dest', choices=['etc', 'downloaded'],
    #                     help="List available services. --list etc lists everything in the \
    #                             /etc/rc.d dir. While installed lists /usr/pkg/share/examples/rc.d")
    parser.add_argument('--test_dir', dest='test_dir', nargs=1, type=str,
                        help="Relative path for you testing purposes")
    list_group.add_argument('--list-rc', dest='rc_services', action='store_true',
                        help="List active services launched from /etc/rc.conf")
    list_group.add_argument('--list-all', dest='active_services', action='store_true',
                        help="Show all service that are activated at startup.")
    parser.add_argument('--show-rc', dest='show_rc_conf', action='store_true',
                        help="Display the /etc/rc.conf file.")

    #

    return parser.parse_args()


def prt_dir(dir_listing: list):
    """setup printing"""
    maxcolumns = 8
    col_width = 0
    padding = 2

    for file_name in dir_listing:
        if len(file_name) > col_width:
            col_width = len(file_name) + padding

    for cnt, f_name in enumerate(sorted(dir_listing), start=1):
        if cnt % maxcolumns == 0:
            print()
        else:
            print(f'{f_name: <{col_width}}', end='')


def process_flag(rc_string):
    print(f"Flag type {rc_string}")


def prt_rc_conf(rc_file):
    with open(rc_file) as f:
        for line in f.readlines():
            print(line.strip())


def lookup_up(rc_string):
    print(rc_string)


def main():
    """main -  it all starts here"""
    args = read_args()
    test_data_dir = (lambda x: ''.join(args.test_dir) if args.test_dir is not None else '')(args)
    rc_data = NetBsdRc(test_data_dir=test_data_dir + '/')

    rc_file_data = rc_data.read_rc_conf()

    etc_rcd_files = rc_data.rc_dot_d_files(rc_data.etc_rc_path)
    example_rcd_files = rc_data.rc_dot_d_files(rc_data.example_rc_path)

    if args.rc_string is not None:

        if not bool(re.search(r'\w.*?=\w.*?', args.rc_string)):
            result = rc_data.service_in_rc_conf(service_key=args.rc_string, file_data=rc_file_data)
            print(result.line_value)
            return

        # rc_data.flags_type = True if '_flags' in args.rc_string else False
        if '_flags' in args.rc_string:
            process_flag(args.rc_string)
            return
        
        service = args.rc_string.split('=')[0]
        value = args.rc_string.split('=')[1]
        result = rc_data.service_in_rc_conf(service_key=service, service_value=value, file_data=rc_file_data)
        if result.found and result.is_same and not result.is_commented:
            print(f"Service {result.line_value} at line {result.line_number}, doing nothing")
            print("bye")
            sys.exit(0)
        elif (result.found and not result.is_same) or (result.found and result.is_commented):
            answer = input(f"You want to change ({result.line_value}) to "
                           f"({args.rc_string})? [y/N]~> ")
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
                    print(f"Bad input format, should take the form of \n\t"
                          f"service=value {rc_data.enabling_value}")

    if args.show_rc_conf:
        prt_rc_conf(rc_file=rc_data.rc_conf_file)

    if args.rc_services:
        rc_data.list_rc_services()

    # if args.dest == 'etc':
    #     prt_dir(etc_rcd_files)
    # elif args.dest == 'downloaded':
    #     prt_dir(example_rcd_files)


if __name__ == '__main__':
    main()
    sys.exit(0)
