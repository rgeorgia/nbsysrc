// use std::env ;
mod usage_help ;

extern crate whoami ;

fn main() {

    println!("Hello, world!");
	usage_help::print_help() ;
	usage_help::print_usage() ;

	println!("Operating System: {}", std::env::consts::OS) ;
	println!("You are on host: {}", whoami::hostname()) ;
	println!("Your username is: {}", whoami::username()) ;

}
