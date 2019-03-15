extern crate whoami ;
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
	// println!("Your username is: {}", whoami::username()) ;
	let matches = App::new("nbsysrc")
                          .version("1.0")
                          .author("Ron Georgia <netverbs@gmail.com>")
                          .about("Simple sysrc for NetBSD")
                          .args_from_usage(
                              "-c, --config=[FILE] 'Sets a custom config file'
                              <INPUT>              'Sets the input file to use'
                              -v...                'Sets the level of verbosity'")
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg_from_usage("-d, --debug 'Print debug information'"))
                          .get_matches(); 
	

} //END
