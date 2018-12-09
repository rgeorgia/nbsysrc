# nbsysrc
This started out to be a simple sysrc for NetBSD; however there may be some room to allow for FreeBSD, HardenedBSD and DragonflyBSD.

Written with Python 3.7+ in mind.

#### NOTE:
Added develop branch to use click instead of argparse

Usage:
```bash
usage: nbsysrc.py [-h] [--list {etc,installed}] [--test_dir TEST_DIR]
                  [-a]
                  [SERVICE=VALUE]

Command line to update rc.conf file

positional arguments:
  SERVICE=VALUE         This is what you want to add

optional arguments:
  -h, --help            show this help message and exit
  --list {etc,installed}
                        List available services. --list etc lists everything
                        in the /etc/rc.d dir. While installed lists
                        /usr/pkg/share/examples/rc.d
  --test_dir TEST_DIR   Relative path for you testing purposes
  -a                    List active services launched from /etc/rc.conf
  ```
  
  nbsysrc.py 
  * ```--list etc``` returns a list of all the services in the /etc/rc.d directory.
  * ```--list installed``` returns a list of the services that have been installed and are in the /usr/{local_d}/share/examples/rc.d/ directory.
   
  **Note**: local_d for NetBSD would be pkg, all others are probably local. For NetBSD we find services we install (like dbus) in ```/usr/pkg/share/examples/rc.d```
  * ```-a``` is not ready yet (WIP)
  * ```service=value``` is the service and and value you want to add to rc.conf.
  
  #### Example:
  ```nbsysrc.py dbus=YES```
  
 Some Details
 ------------
 
 Check out our [wiki](https://github.com/rgeorgia/nbsysrc/wiki) for more details.