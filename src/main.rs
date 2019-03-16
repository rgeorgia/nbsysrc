extern crate whoami ;
#[macro_use]
extern crate clap ;

use std::env ;
use std::io::{Write} ;
use std::str::FromStr ;
use std::process ;
use clap::{Arg, App, SubCommand} ;

mod usage_help ;

fn main() {

	// usage_help::print_help() ;
	// usage_help::print_usage() ;
	// println!("Operating System is: {}", whoami::os()) ;
	// println!("Operating System for platforms: {:?}", std::env::consts::OS) ;
	// println!("The environment is: {}", whoami::env()) ;
	// println!("You are on host: {}", whoami::hostname()) ;
	println!("Your username is: {}", whoami::username()) ;

} //END
