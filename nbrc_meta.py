import platform


# TODO: consider using factory pattern that will allow the return of the proper RC class, based on the OS

class RcMetaData:
    def __init__(self, test_data_dir: str, debug_test: bool):

        self.debug_test = debug_test
        self.test_data_dir = test_data_dir

        if platform.system().lower() == 'netbsd' or platform.system().lower() == 'darwin':
            self.local_d = 'pkg'
        else:
            # This should take care of dragonfly, FreeBSD, HardenBSD
            self.local_d = 'local'

        if self.debug_test:
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


class NetBsdRc(RcMetaData):

    _flags_types: bool

    def __init__(self, debug_test: bool = False, test_data_dir: str = 'data/'):
        super().__init__(debug_test=debug_test, test_data_dir=test_data_dir)

        self._flags_types: bool

    def __repr__(self):
        return f"{self.__class__}, {self.flag_types}, "

    @property
    def flag_types(self):
        return self._flags_types

    @flag_types.setter
    def flag_types(self, flag_t: bool):
        self._flags_types = flag_t

    @property
    def enabling_value(self):
        return ['YES', 'TRUE', 'ON', '1', 'NO', 'FALSE', 'OFF', '0']
