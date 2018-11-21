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
import sys
import argparse
# import platform
from pathlib import Path
from typing import NamedTuple
from collections import namedtuple


class RcMetaData(NamedTuple):
    etc_rc_path: str = "data/etc/rc.d/"
    example_rc_path: str = "data/usr/pkg/share/examples/rc.d/"
    rc_conf_file: str = "data/etc/rc.conf"
    rc_local: str = "data/etc/rc.local"


rc_data = RcMetaData()


def read_args():
    parser = argparse.ArgumentParser(description="Command line to update rc.conf file")
    parser.add_argument('rc_string', nargs='?', metavar='STRING TO ADD', help="This is what you want to add")
    parser.add_argument('--list', dest='dest', choices=['etc', 'example'],
                        help="List available services")

    return parser.parse_args()


def read_rc_conf():
    with open(rc_data.rc_conf_file) as f:
        data = f.readlines()

    return data


def check_format(input_str: str):
    pass


def rc_dot_d_files(rc_d_dir: str):
    p = Path(rc_d_dir)
    file_list = [x.name for x in p.iterdir()]

    return file_list


def service_in_rc_conf(service: str, file_data: list):
    result = namedtuple('result', 'found line_number current_status desired_status')
    result.found = False

    for line_num, line in enumerate(file_data):
        if service.split('=')[0] == line.split('=')[0]:
            result.found = True
            result.line_number = line_num + 1
            result.current_status = line.split('=')[1].strip()
            result.desired_status = service.split('=')[1].strip()

    return result


def add_line(rc_string):
    pass


def change_line(service: str, result: namedtuple):
    print(
        f"Changing {service.replace(service.split('=')[1], result.current_status.upper())} to "
        f"{service} at line {result.line_number}")


def prt_dir(dir_listing: list):
    MAXCOLUMNS = 8
    col_width = 0
    padding = 2

    for y in dir_listing:
        if len(y) > col_width:
            col_width = len(y) + padding

    for cnt, x in enumerate(sorted(dir_listing), start=1):
        if cnt % MAXCOLUMNS == 0:
            print()
        else:
            print(f'{x: <{col_width}}', end='')


def check_input_format(rc_string: str):
    pass


def main():
    rc_file_data = read_rc_conf()
    args = read_args()
    if args.rc_string is not None:
        result = service_in_rc_conf(service=args.rc_string, file_data=rc_file_data)
        if result.found and result.current_status.upper() == result.desired_status.upper():
            print(f"Service {args.rc_string} at line {result.line_number}, doing nothing")
            print("bye")
            sys.exit(0)
        elif result.found and result.current_status.upper() != result.desired_status.upper():
            answer = input(f"You want to change {result.current_status.upper()} to "
                           f"{result.desired_status.upper()}? [y/N]~> ").lower()
            if answer == 'y':
                change_line(args.rc_string, result)
        else:
            answer = input(f"Add {args.rc_string} to rc.conf? [y/N]~> ").lower()
            if answer == 'y':
                check_input_format(args.rc_string)
                add_line(args.rc_string)

    if args.dest == 'etc':
        prt_dir(rc_dot_d_files(rc_data.etc_rc_path))
    elif args.dest == 'example':
        prt_dir(rc_dot_d_files(rc_data.example_rc_path))


if __name__ == '__main__':
    main()
