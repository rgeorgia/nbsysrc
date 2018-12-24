# nbsysrc
This started out to be a simple sysrc for NetBSD; however there may be some room to allow for FreeBSD, HardenedBSD and DragonflyBSD.

Written with Python 3.6+ in mind.

I am tired of opening rc.conf with vi in order to add a service, or a flag. I don’t want to have to “cat” the rc.conf to see the rc status of a or flag.

I need a single application that does it for me, something similar to sysrc used by FreeBSD and it’s derivatives. Basically nbsysrc will take an argument from the command line and insert or append it to the rc.conf file. 

  #### Example:
  ```nbsysrc.py dbus=YES```
  
 Some Details
 ------------
 
 Check out our [wiki](https://github.com/rgeorgia/nbsysrc/wiki) for more details.

 Check out our [Roadmap wiki](https://github.com/rgeorgia/nbsysrc/wiki/Roadmap) for more details.
