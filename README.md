# nbsysrc
This started out to be a simple sysrc for NetBSD; however there may be some room to allow for FreeBSD, HardenedBSD and DragonflyBSD.

Written with Python 3.6+ in mind.

Usage:
```bash
nbsysrc.py --help
usage: nbsysrc.py [-h] [--list {etc,installed}] [-a] [SERVICE=VALUE]

Command line to update rc.conf file

positional arguments:
  SERVICE=VALUE         This is what you want to add

optional arguments:
  -h, --help            show this help message and exit
  --list {etc,installed}
                        List available services
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
 
1. Enter service=value pair
1. nbsysrc verifies the input is a valid format using a little regex
1. If the service and value is already in rc.conf, we alert the user that there is nothing to do and exit.
1. It the service is in the rc.conf file, but the value is different, we ask the user if they want to change the value, if yes we change that line, if not, we exit.
1. If the service entered is not in ```/etc/rc.d``` or in ```/usr/{local_d}/share/examples/rc.d/``` then we tell the user that the service they want to add to rc.conf is not available and needs to be installed. 
    1. This will need to change in order to account for entering things like hostname= or service flags.
1. If the service is in ```/usr/{local_d}/share/examples/rc.d/``` but not in ```/etc/rc.d```, then we alert the user, telling them they might want to copy the file to ```/etc/rc.d```.
    1. See subpoint for #5.
1. Finally, if everything checks out we check the format and add the line to rc.conf. 