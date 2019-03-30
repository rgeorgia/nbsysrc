use std::env ;
use std::fs ;

#[derive(Debug, Copy, PartialEq)]
pub struct RcConfFile {
    path_location: String,
    content: String
}