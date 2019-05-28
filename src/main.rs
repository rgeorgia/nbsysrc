// #[macro_use]
extern crate clap;
extern crate nix;

use clap::{App, Arg, ArgGroup};
use std::process::Command;
use std::process ;
use std::fs ;
use std::path::Path ;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct ConfFile {
    dir_location: String, 	// fully qualified path
    name: String,   	    // name of the file, defaults to rc.conf
    content: String,        // contents of the file
}

fn main() {
    let rc_file_name = "rc.conf" ;
    let default_dir = "/etc" ;

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

    if !Path::new(&format!("{}/{}",default_dir, rc_file_name)).exists() && !matches.is_present("test-dir") {
        println!("Looks like {} does not exist.", format!("{}/{}", default_dir, rc_file_name)) ;
        process::exit(1);
    }

    let rc_file = if matches.is_present("test-dir") {
        build_rc_file(matches.value_of("test-dir")
            .unwrap(),rc_file_name)
    } else {
        build_rc_file(&default_dir.to_string(),&rc_file_name.to_string())
    } ;

    if matches.is_present("showrc") {
        println!("{}", rc_file.content) ;

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
        let value: Vec<&str> = matches.value_of("service").unwrap().split("=").collect() ;
        if !is_valid_service(&value[1]) {
            println!("{} is not a valid value", &value[1]) ;
            process::exit(1) ;
        }
    }

    println!("Done, line added") ;
} //END MAIN

fn build_rc_file(dir_location: &str, name: &str) -> ConfFile {
    ConfFile {
        dir_location: dir_location.to_string(),
        name: name.to_string(),
        content: read_file(format!("{}/{}", dir_location, name))
    }
}


fn is_valid_service(value: &str) -> bool {
    match value {
        "YES" | "NO" | "TRUE" | "FALSE" | "ON" | "OFF" | "0" | "1" => true,
        _ => false,
    }
} // end is_valid_service

fn read_file(file_with_path: String) -> String {
    //read the file and return the content
    fs::read_to_string(file_with_path)
        .expect("Could not read file")
} //end read_file

#[cfg(test)]
mod tests {
    use super::* ;

    #[test]
    fn test_is_valid_service() {
        assert!(is_valid_service("YES")) ;
    }

    #[test]
    fn test_is_not_valid_service() {
        assert_eq!(is_valid_service("NOPE"), false) ;
    }

}
