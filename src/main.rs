// #[macro_use]
extern crate clap;
extern crate nix;

use clap::{App, Arg, ArgGroup};
use std::process::Command;
use crate::nbrc::RcConfFile;

mod nbrc ;

fn main() {
    let mut enabling: bool = true;

    let matches = App::new("cli-args")
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

    if matces.value_of("test-dir").unwrap() {
        let rc_file = RcConfFile {
            location: String::from(matches.value_of("test-dir").unwrap()),
            name: String::from("rc.conf")

        } ;
    }
    if matches.value_of("service").unwrap().contains(&"flag") {
        enabling = false;
        println!(
            "You selected a flag type entry, {} {}",
            matches.value_of("service").unwrap(),
            enabling
        );
    } else {
        println!(
            "You selecte a service type entry, {} {}",
            matches.value_of("service").unwrap(),
            enabling
        )
        // enabling = true ;
    }

    if is_netbsd() {
        println!("I am BSD of type: {}", get_os_bsd());
        println!("My version is: {}", get_bsd_version());
    }
} //END MAIN

fn get_os_bsd() -> String {
    let nb_output = Command::new("uname")
        .arg("-s")
        .output()
        .expect("failed to execute uname");

    let result = String::from_utf8_lossy(&nb_output.stdout);
    result.to_string()
}


fn get_bsd_version() -> String {
    let nb_output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("failed to execute uname");

    String::from_utf8_lossy(&nb_output.stdout).to_string()
}

fn is_netbsd() -> bool {
    let output = Command::new("uname")
        .arg("-s")
        .output()
        .expect("failed to execute uname -s");
    if String::from_utf8_lossy(&output.stdout).contains("BSD") {
        true
    } else {
        false
    }
}

fn show_rc() {

}
