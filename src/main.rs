extern crate whoami ;
#[macro_use]
extern crate clap ;

use clap::{Arg, App} ;

fn main() {
	let mut enabling = true  ;

	let matches = App::new(crate_name!())
    	.about("sysrc for NetBSD")
       	.author(crate_authors!())
	   	.arg(Arg::with_name("service")
	   		.help("Key=value pair. Ex dbus=YES")
			.index(1)
	   		.required(true))
		.arg(Arg::with_name("show-rc")
			.help("Show the contents of rc.conf file")
			.conflicts_with("service")
			.long("show-rc"))
		.arg(Arg::with_name("list-rc")
			.help("List active services launched from /etc/rc.conf")
			.conflicts_with("service")
			.long("list-rc"))
		.arg(Arg::with_name("list-all")
			.help("Show all service that are activated at startup.")
			.conflicts_with("service")
			.long("list-all"))
		.arg(Arg::with_name("list")
		// need to add choices [etc or installed]
			.help("List available services. --list etc lists everything in the /etc/rc.d dir. 
While installed lists /usr/pkg/share/examples/rc.d\n")
			.long("list")
			.conflicts_with("service")
			.possible_values(&["etc", "installed"])
			.takes_value(true))
		.arg(Arg::with_name("test-dir")
		// requires value
			.help("Relative path for you testing purposes")
			.takes_value(true)
			.long("test-dir"))
       .get_matches() ;

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
