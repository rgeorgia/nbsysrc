extern crate whoami ;
#[macro_use]
extern crate clap ;

use clap::{Arg, App, ArgGroup} ;

fn main() {
	let mut enabling = true  ;

    let matches = App::new("args-ex")
                .group(ArgGroup::with_name("flags")
                    .required(true)
                )
                .arg(Arg::with_name("service")
	   		        .help("Key=value pair. Ex dbus=YES")
			        .index(1)
                    .group("flags"))
                .arg(Arg::with_name("show-rc")
                    .long("show-rc")
                    .help("Show the contents of rc.conf file")
                    .group("flags"))
                .arg(Arg::with_name("list-rc")
                    .long("list-rc")
                    .help("List active services launched from /etc/rc.conf")
                    .group("flags"))
                .arg(Arg::with_name("list-all")
                    .help("Show all service that are activated at startup.")
                    .long("list-all")
                    .group("flags"))
                .arg(Arg::with_name("list")
                    .help("List available services. --list etc lists everything in the /etc/rc.d dir.\nWhile installed lists /usr/pkg/share/examples/rc.d\n")
                    .long("list")
                    .possible_values(&["etc", "installed"])
                    .takes_value(true)
                    .group("flags"))
                .arg(Arg::with_name("test-dir")
                    .long("test-dir")
                    .value_name("TEST_DIR")
                    .takes_value(true))
                .get_matches();

	if matches.value_of("service").unwrap().contains(&"flag") {
		enabling = false ;
		println!("You selected a flag type entry, {} {}",  matches.value_of("service").unwrap(), enabling) ;
		
	} else {
		println!("You selecte a service type entry, {} {}", matches.value_of("service").unwrap(), enabling)
		// enabling = true ;
	}
	profile() ;
} //END

fn profile() {
	println!("Operating System is: {}", whoami::os()) ;
	println!("Operating System for platforms: {:?}", std::env::consts::OS) ;
	println!("The environment is: {}", whoami::env()) ;
	println!("You are on host: {}", whoami::hostname()) ;
	println!("Your username is: {}", whoami::username()) ;
}
