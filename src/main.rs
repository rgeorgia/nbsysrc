// #[macro_use]
extern crate clap ;
extern crate os_type ;

use clap::{Arg, App, ArgGroup} ;
use std::process::Command;

#[derive(Debug)]
struct RcServiceDirs {
	etc_rcd_path: String,
	example_rcd_path: String, 
}

impl RcServiceDirs {
	fn new(etc_rcd_path: String, example_rcd_path: String)-> RcServiceDirs {
		RcServiceDirs {etc_rcd_path, example_rcd_path }
	}
}

fn main() {
	let mut enabling = true  ;

    let matches = App::new("args-ex")
	// The service arg and the options are mutually exclusive. The only "option" allowed with either is 
	// the test-dir option.
                .group(ArgGroup::with_name("flags")
                    .required(true))
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

	get_local_path() ;
	println!("{}", is_netbsd()) ;

} //END MAIN

fn get_local_path() {

    let nb_output = Command::new("uname").arg("-s").output()
        .expect("failed to execute uname") ;
    
    println!("Status: {}", nb_output.status) ;
    println!("stdout: {}", String::from_utf8_lossy(&nb_output.stdout)) ;
    println!("stderr: {}", String::from_utf8_lossy(&nb_output.stderr)) ;
}

fn is_netbsd() -> bool {
    match Command::new("uname -s").output() {
        Ok(output) =>
                output.status.success() ,
        Err(_) => false
    }
}
