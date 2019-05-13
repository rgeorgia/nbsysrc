use std::fs ;

// allowable values

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct ConfFile {
	pub dir_location: String, 	// fully qualified path
	pub name: String,   	// name of the file, defaults to rc.conf
	pub content: String,    // contents of the file
}

// data or properties
impl ConfFile {
	pub fn new(location: &str, name: &str) -> ConfFile {
		ConfFile {
			dir_location: location.to_string(),
			name: name.to_string(),
			content: ConfFile::read_file(format!("{}/{}", &location, &name))
		}
	}

	fn read_file(file_with_path: String) -> String {
		//read the file and return the content
		fs::read_to_string(file_with_path)
			.expect("Could not read file")
	}
}

#[allow(dead_code)]
impl ConfFile {
	fn fqn(&self) -> String {
		format!("{}/{}", self.dir_location, self.name)
	}

}
