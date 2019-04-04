use std::env ;
use std::fs ;


#[derive(Debug, Copy, PartialEq)]
pub struct RcConfFile {
    location: String,	// fully qualified path
	name: String, 		// name of the file, defaults to rc.conf
	content: String, 	// contents of the file
	permissions: u16,	// file permissions, default is 644
	owner: String,		// file owner, default is root
	group: String		// owning group, default is wheel

}

// data or properties
impl RcConfFile {
	fn fqn(&self) -> String {
		self.location + self.name
	}
}
// methods
impl RcConfFile {
	fn read_file(&self) -> String {
		//read the file and return the content
		String::from("file content")
	}
}