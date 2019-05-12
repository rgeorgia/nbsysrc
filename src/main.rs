// #[macro_use]
extern crate clap;
extern crate nix;

use clap::{App, Arg, ArgGroup};
use std::process::Command;
use crate::nbrc::RcConfFile;

mod nbrc ;

fn main() {
//    let rc_content = read_file("/etc/rc.conf") ;
//    let mut nbrc = RcConfFile {
//        location: "/etc/".to_string(),
//        name: "rc.conf".to_string(),
//        content: rc_content.to_string()
//    } ;
    let nbrc = RcConfFile::new("/etc".to_string(),
                                   "rc.conf".to_string()) ;

    let matches = App::new("cli-args")
                .author("Ronverbs")
                .version("1.0.0")
                .about("Edits rc.conf file.")
                .group(ArgGroup::with_name("flags")
                    .required(true))
                .arg(Arg::with_name("service")
	   		        .help("Key=value pair. Ex dbus=YES")
			        .index(1)
                    .group("flags"))
                .arg(Arg::with_name("showrc")
                    .long("showrc")
                    .help("Show the contents of rc.conf file")
                    .group("flags"))
                .arg(Arg::with_name("test-dir")
                    .long("test-dir")
                    .value_name("TEST_DIR")
                    .takes_value(true))
                .get_matches();

    if matches.is_present("showrc") {
        println!("rc.conf content");
    }

    else if matches.value_of("service").unwrap().contains(&"flag") {
        println!(
            "You selected a flag type entry, {}",
            matches.value_of("service").unwrap()
        );
    } else {
        println!(
            "You selecte a service type entry, {}",
            matches.value_of("service").unwrap()
        ) ;
    }

    fn is_valid_service(value: &str) -> bool {
        match value {
            "YES" | "NO" | "TRUE" | "FALSE" | "ON" | "OFF" | "0" | "1" => true,
            _ => false,
        }
    } // end is_valid_service


} //END MAIN

//fn read_file(file_with_path: &str) -> String {
//    //read the file and return the content
//    let contents = fs::read_to_string(file_with_path)
//                            .expect("Could not read file") ;
//    contents
//}

#[allow(dead_code)]
fn get_os_bsd() -> String {
    let nb_output = Command::new("uname")
        .arg("-s")
        .output()
        .expect("failed to execute uname");

    let result = String::from_utf8_lossy(&nb_output.stdout);
    result.to_string()
}

#[allow(dead_code)]
fn get_bsd_version() -> String {
    let nb_output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("failed to execute uname");

    String::from_utf8_lossy(&nb_output.stdout).to_string()
}

#[allow(dead_code)]
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

