#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod igs {
    use super::*;
    pub fn version() -> i32 {
        unsafe { return igs_version() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn test_simple_igs_version() {
        unsafe {
            assert_eq!(igs_version(), 040105);
        }
    }

    #[test]
    fn test_version() {
        assert_eq!(igs::version(), 040105);
    }

    fn make_safe_string_from_c(c_buf: *mut i8) -> String {
        unsafe { CStr::from_ptr(c_buf).to_string_lossy().into_owned() }
    }

    #[test]
    fn test_net_devices_and_addresses() {
        unsafe {
            let mut s1 = 0;
            let mut s2 = 0;
            igs_net_devices_list(ptr::addr_of_mut!(s1));
            igs_net_addresses_list(ptr::addr_of_mut!(s2));
            assert_eq!(s1, s2);
        }
    }

    #[test]
    fn test_agent_name() {
        unsafe {
            let str1 = make_safe_string_from_c(igs_agent_name());
            assert_eq!(str1, "no_name");
            let c_str = CString::new("simple Demo Agent").expect("CString::new failed");
            igs_agent_set_name(c_str.as_ptr());
            let str2 = make_safe_string_from_c(igs_agent_name());
            assert_eq!(str2, "simple_Demo_Agent");
        }
    }

    #[test]
    fn test_agent_family() {
        unsafe {
            let c_str: *mut i8 = igs_agent_family();
            assert!(c_str.is_null());
            let c_str = CString::new("family_test").expect("CString::new failed");
            igs_agent_set_family(c_str.as_ptr());
            let c_str: *mut i8 = igs_agent_family();
            let str = make_safe_string_from_c(c_str);
            assert_eq!(str, "family_test");
        }
    }

    #[test]
    fn test_igs_logs() {
        unsafe {
            assert!(!igs_log_console());
            igs_log_set_console(true);
            assert!(igs_log_console());

            assert!(!igs_log_syslog());
            igs_log_set_syslog(true);
            assert!(igs_log_syslog());

            assert!(!igs_log_console_color());
            igs_log_set_console_color(true);
            assert!(igs_log_console_color());

            assert!(!igs_log_stream());
            igs_log_set_stream(true);
            assert!(igs_log_stream());

            assert!(!igs_log_file());
            assert!(igs_log_file_path().is_null());
            let c_str = CString::new("/tmp/log.txt").expect("CString::new failed");
            igs_log_set_file_path(c_str.as_ptr());
            let path = make_safe_string_from_c(igs_log_file_path());
            assert_eq!(path, "/tmp/log.txt");

            igs_log_set_console_level(igs_log_level_t_IGS_LOG_TRACE);
            assert_eq!(igs_log_console_level(), igs_log_level_t_IGS_LOG_TRACE);
        }
    }

    #[test]
    fn test_mapping_requests() {
        unsafe {
            assert!(!igs_mapping_outputs_request());
            igs_mapping_set_outputs_request(true);
            assert!(igs_mapping_outputs_request());
        }
    }

    #[test]
    fn test_agent_state() {
        unsafe {
            let c_str = CString::new("my state").expect("CString::new failed");
            igs_agent_set_state(c_str.as_ptr());
            assert_eq!(make_safe_string_from_c(igs_agent_state()), "my state");
        }
    }

    #[test]
    fn test_agent_mute() {
        unsafe {
            assert!(!igs_agent_is_muted());
            igs_agent_mute();
            assert!(igs_agent_is_muted());
            igs_agent_unmute();
            assert!(!igs_agent_is_muted());
        }
    }

    #[test]
    fn test_agent_frozen() {
        unsafe {
            assert!(!igs_is_frozen());
            igs_freeze();
            assert!(igs_is_frozen());
            igs_unfreeze();
            assert!(!igs_is_frozen());
        }
    }
}
