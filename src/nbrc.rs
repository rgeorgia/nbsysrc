use std::fs ;

// allowable values

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct RcConfFile {
	location: String, 	// fully qualified path
	name: String,   	// name of the file, defaults to rc.conf
	content: String,    // contents of the file
}

// data or properties
impl RcConfFile {
	pub fn new(location: String, name: String) -> RcConfFile {
		RcConfFile {
			location,
			name,
			content: RcConfFile::read_file(format!("{}/{}", &location, &name))
		}
	}

	fn read_file(file_with_path: String) -> String {
		//read the file and return the content
		fs::read_to_string(file_with_path)
			.expect("Could not read file")
	}
}

#[allow(dead_code)]
impl RcConfFile {
	fn fqn(&self) -> String {
		format!("{}/{}", self.location, self.name)
	}


}
