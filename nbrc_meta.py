import fileinput
import platform

# TODO: add try block to write to rc.conf because you need to be root
import re
from collections import namedtuple
from pathlib import Path


class NetBsdRc:
    """base class """
    def __init__(self,  test_data_dir: str = '/'):

        self.test_data_dir = test_data_dir
        self.local_d = 'pkg'

        if self.test_data_dir is not '/':
            self.root_dir = self.test_data_dir
        else:
            self.root_dir = '/'

    @property
    def etc_rc_path(self):
        return f"{self.root_dir}etc/rc.d/"

    @property
    def example_rc_path(self):
        return f"{self.root_dir}usr/{self.local_d}/share/examples/rc.d/"

    @property
    def rc_conf_file(self):
        return f"{self.root_dir}etc/rc.conf"

    @property
    def rc_local(self):
        return f"{self.root_dir}etc/rc.local"

    def check_input_format(self, rc_string: str):
        pass

    def read_rc_conf(self):
        with open(self.rc_conf_file) as f_name:
            data = f_name.readlines()
        return data

    @staticmethod
    def rc_dot_d_files(rc_d_dir: str):
        """List files in /etc/rc.d"""
        p = Path(rc_d_dir)
        return [x.name for x in p.iterdir()]


    @staticmethod
    def service_in_rc_conf(service: str, file_data: list):
        # TODO: find matching lines, but that have been commented out
        result_fields = 'found line_number line_value current_status desired_status is_same is_commented'
        result = namedtuple('result', result_fields)
        result.__new__.__defaults__ = (False, None, None, None, None, False, False)

        for line_num, line in enumerate(file_data):
            if service.split('=')[0] in line.split('=')[0]:
                result.found = True
                result.line_number = line_num + 1
                result.current_status = line.split('=')[1].strip()
                result.desired_status = service.split('=')[1].strip()
                result.line_value = line.strip()
                result.is_same = True if service.split('=')[0] == line.split('=')[0] else False
                result.is_commented = True if "#" in line.split('=')[0] else False
                break

        return result

    def replace_line(self, to_replace: str, replacement: str):
        with fileinput.input(files=self.rc_conf_file, inplace=True, backup='.bak') as f:
            for line in f:
                if to_replace in line:
                    print(replacement.strip())
                else:
                    print(line.strip())

    def add_line(self, rc_string: str):
        with open(self.rc_conf_file, "a") as f:
            f.write(rc_string)

    @property
    def flag_types(self):
        return self._flags_types

    @flag_types.setter
    def flag_types(self, flag_t: bool):
        self._flags_types = flag_t

    @property
    def enabling_value(self):
        return ['YES', 'TRUE', 'ON', '1', 'NO', 'FALSE', 'OFF', '0']

    def check_input_format(self, rc_string: str) -> bool:

        if not bool(re.search(r'\w.*?=\w.*?', rc_string)):
            return False
        if rc_string.split('=')[1] not in self.enabling_value:
            return False

        return True
