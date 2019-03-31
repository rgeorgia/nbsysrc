use std::env ;
use std::fs ;


#[derive(Debug, Copy, PartialEq)]
pub struct RcConfFile {
    rc_conf_path: String,
	etc_rcd_path: String,
	example_rcd_path: String,
	rc_conf_name: String,
	rc_local_name: String
}

impl RcConfFile {
	const RC_CONF_PATH: String = "/etc".to_string() ;
	const ECT_RCD_PATH: String = "/etc/rc.d".to_string() ;
	const EXAMPLE_RCD_PATH: String = "/usr/pkg/share/examples/rc.d".to_string() ;
	const RC_CONF_NAME: String = "rc.conf".to_string() ;
	const RC_LOCAL_NAME: String = "rc.local".to_string() ;

	fn default() -> RcConfFile {
		RcConfFile {
			rc_conf_path: RC_CONF_PATH:,
			etc_rcd_path: ECT_RCD_PATH,
			example_rcd_path: EXAMPLE_RCD_PATH,
			rc_conf_name: RC_CONF_NAME,
			rc_local_name: RC_LOCAL_NAME
		}
	} // end default

	fn with_test(test_dir: &str) -> RcConfFile {
		RcConfFile {
			rc_conf_path: concat!(&test_dir, RC_CONF_PATH),
			etc_rcd_path: concat!(&test_dir, ECT_RCD_PATH),
			example_rcd_path: concat!(&test_dir, EXAMPLE_RCD_PATH),
			rc_conf_name: RC_CONF_NAME,
			rc_local_name: RC_LOCAL_NAME
		}
	} // end with_test
} // end imp RcConfFile

impl RcConfFile {
	fn read_file(&self, file_name: &str) -> String {
		// read file_name and return contents
		
	}
}
