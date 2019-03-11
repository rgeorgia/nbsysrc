// use std::env ;
mod usage_help ;

extern crate whoami ;

fn main() {

    println!("Hello, world!");
	usage_help::print_help() ;
	usage_help::print_usage() ;

	println!("Operating System is: {}", whoami::os()) ;
	println!("Operating System for platforms: {:?}", std::env::consts::OS) ;
	println!("The environment is: {}", whoami::env()) ;
	println!("You are on host: {}", whoami::hostname()) ;
	println!("Your username is: {}", whoami::username()) ;

}
