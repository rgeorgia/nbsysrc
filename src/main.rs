extern crate whoami ;
#[macro_use]
extern crate clap ;

use clap::{Arg, App, SubCommand} ;

fn main() {
	let mut enabling = true  ;

	let matches = App::new("nbsysrc")
    	.about("does awesome things")
       	.author(crate_authors!())
	   	.arg(Arg::with_name("service")
	   		.help("Key=value pair. Ex dbus=YES")
	   		.required(true))
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