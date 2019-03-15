use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;


mod usage_help ;

extern crate whoami ;

 struct Arguments {
     key_value: String,
	 list: String,
	 test_dir: String,
	 list_rc: bool,
	 list_all: bool,
	 show_rc: bool,
	 interactive:bool,    
 }

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        } else {
			println!("There are {} arguments. {:?}", args.len(), args) ;
			return Err("something is wrong") ;
		}

    }
}


fn main() {

	// usage_help::print_help() ;
	// usage_help::print_usage() ;
	// println!("Operating System is: {}", whoami::os()) ;
	// println!("Operating System for platforms: {:?}", std::env::consts::OS) ;
	// println!("The environment is: {}", whoami::env()) ;
	// println!("You are on host: {}", whoami::hostname()) ;
	// println!("Your username is: {}", whoami::username()) ;

	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();
	let arguments = Arguments::new(&args).unwrap_or_else(
		|err| {
			if err.contains("help") {
				process::exit(0);
			} else {
				eprintln!("{} problem parsing arguments: {}", program, err);
				process::exit(0);
			}
		}
	); //END let arguments


} //END
