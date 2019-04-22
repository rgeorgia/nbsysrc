use std::fs ;

// allowable values

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct RcConfFile {
    pub location: String,	// fully qualified path
	pub name: String, 		// name of the file, defaults to rc.conf
	content: String, 	// contents of the file

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
	pub fn read_file(&mut self) -> String {
		//read the file and return the content
		let mut contents = fs::read_to_string(self.fqn())
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
