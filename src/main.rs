use os_type ;
use std::env::consts::OS ;
mod usage_help ;

extern crate whoami ;

fn main() {
	//let os = os_type::current_platform() ;

    println!("Hello, world!");
	usage_help::print_help() ;
	usage_help::print_usage() ;

	println!("Operating System is: {}", whoami::os()) ;
	println!("Operating System for platforms: {:?}", std::env::consts::OS) ;
	println!("The environment is: {}", whoami::env()) ;
	println!("You are on host: {}", whoami::hostname()) ;
	println!("Your username is: {}", whoami::username()) ;

}
