use std::env ;
use std::fs ;

#[derive(Debug, Copy, PartialEq)]
pub struct RcConfFile {
    path_location: String,
    content: String
}

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