use std::fs ;

// allowable values

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct RcConfFile {
    location: String,	// fully qualified path
	name: String, 		// name of the file, defaults to rc.conf
	content: String, 	// contents of the file
	permissions: u16,	// file permissions, default is 644
	owner: String,		// file owner, default is root
	group: String		// owning group, default is wheel

}

// data or properties
#[allow(dead_code)]
impl RcConfFile {
	fn fqn(&self) -> String {
		format!("{}/{}", self.location, self.name)
	}
}
// methods
#[allow(dead_code)]
impl RcConfFile {
	fn read_file(&mut self) -> String {
		//read the file and return the content
		let contents = fs::read_to_string(self.fqn())
			.expect("Could not read file") ;
		contents
	}
}

// functions, not associated
#[allow(dead_code)]
impl RcConfFile {
	fn is_valid_service(value: &str) -> bool {
    	match value {
        	"YES" | "NO" | "TRUE" | "FALSE" | "ON" | "OFF" | "0" | "1" => true,
        	_ => false,
    	}
	} // end is_valid_service

}