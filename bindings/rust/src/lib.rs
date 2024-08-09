#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod igs {
    use super::*;
    use std::ffi::CStr;
    use std::ffi::CString;

    pub const LOG_TRACE: igs_log_level_t = igs_log_level_t_IGS_LOG_TRACE;
    pub const LOG_DEBUG: igs_log_level_t = igs_log_level_t_IGS_LOG_DEBUG;
    pub const LOG_INFO: igs_log_level_t = igs_log_level_t_IGS_LOG_INFO;
    pub const LOG_WARN: igs_log_level_t = igs_log_level_t_IGS_LOG_WARN;
    pub const LOG_ERROR: igs_log_level_t = igs_log_level_t_IGS_LOG_ERROR;
    pub const LOG_FATAL: igs_log_level_t = igs_log_level_t_IGS_LOG_FATAL;

    fn make_safe_string_from_c(c_buf: *mut i8) -> String {
        unsafe {
            return CStr::from_ptr(c_buf).to_string_lossy().into_owned();
        }
    }

    pub fn version() -> i32 {
        unsafe { return igs_version() }
    }

    pub fn agent_name() -> String {
        unsafe {
            return make_safe_string_from_c(igs_agent_name());
        }
    }

    pub fn agent_set_name(name: &str) {
        let c_str = CString::new(name).expect("CString::new failed");
        unsafe {
            igs_agent_set_name(c_str.as_ptr());
        }
    }

    pub fn agent_family() -> Option<String> {
        unsafe {
            let fam_c: *mut i8 = igs_agent_family();
            if fam_c.is_null() {
                return None;
            }
            return Some(make_safe_string_from_c(fam_c));
        }
    }

    pub fn agent_set_family(family: &str) {
        let c_str = CString::new(family).expect("CString::new failed");
        unsafe {
            igs_agent_set_family(c_str.as_ptr());
        }
    }

    pub fn log_console() -> bool {
        unsafe {
            return igs_log_console();
        }
    }

    pub fn log_set_console(b: bool) {
        unsafe {
            igs_log_set_console(b);
        }
    }

    pub fn log_syslog() -> bool {
        unsafe {
            return igs_log_syslog();
        }
    }

    pub fn log_set_syslog(b: bool) {
        unsafe {
            igs_log_set_syslog(b);
        }
    }

    pub fn log_console_color() -> bool {
        unsafe {
            return igs_log_console_color();
        }
    }

    pub fn log_set_console_color(b: bool) {
        unsafe {
            igs_log_set_console_color(b);
        }
    }

    pub fn log_stream() -> bool {
        unsafe {
            return igs_log_stream();
        }
    }

    pub fn log_set_stream(b: bool) {
        unsafe {
            igs_log_set_stream(b);
        }
    }

    pub fn log_file() -> bool {
        unsafe {
            return igs_log_file();
        }
    }

    pub fn log_file_path() -> Option<String> {
        unsafe {
            let path_c: *mut i8 = igs_log_file_path();
            if path_c.is_null() {
                return None;
            }
            return Some(make_safe_string_from_c(path_c));
        }
    }

    pub fn log_set_file_path(path: &str) {
        let c_str = CString::new(path).expect("CString::new failed");
        unsafe {
            igs_log_set_file_path(c_str.as_ptr());
        }
    }

    pub fn log_console_level() -> igs_log_level_t {
        unsafe {
            return igs_log_console_level();
        }
    }

    pub fn log_set_console_level(level: igs_log_level_t) {
        unsafe {
            igs_log_set_console_level(level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::ptr;

    fn make_safe_string_from_c(c_buf: *mut i8) -> String {
        unsafe {
            return CStr::from_ptr(c_buf).to_string_lossy().into_owned();
        }
    }

    #[test]
    fn test_version() {
        assert_eq!(igs::version(), 040105);
    }

    #[test]
    fn test_agent_name() {
        assert_eq!(igs::agent_name(), "no_name");
        igs::agent_set_name("simple Demo Agent");
        assert_eq!(igs::agent_name(), "simple_Demo_Agent");
    }

    #[test]
    fn test_agent_family() {
        assert_eq!(igs::agent_family(), None);
        igs::agent_set_family("family_test");
        assert_eq!(igs::agent_family(), Some("family_test".to_string()));
    }

    //
    // UNSAFE
    //

    #[test]
    fn test_igs_logs() {
        assert!(!igs::log_console());
        igs::log_set_console(true);
        assert!(igs::log_console());

        assert!(!igs::log_syslog());
        igs::log_set_syslog(true);
        assert!(igs::log_syslog());

        assert!(!igs::log_console_color());
        igs::log_set_console_color(true);
        assert!(igs::log_console_color());

        assert!(!igs::log_stream());
        igs::log_set_stream(true);
        assert!(igs::log_stream());

        assert!(!igs::log_file());
        assert_eq!(igs::log_file_path(), None);
        igs::log_set_file_path("/tmp/log.txt");
        assert_eq!(igs::log_file_path(), Some("/tmp/log.txt".to_string()));

        // Here we test all LOG_XXX constants so they are used somewhere and no unused warning is raised
        igs::log_set_console_level(igs::LOG_TRACE);
        assert_eq!(igs::log_console_level(), igs::LOG_TRACE);
        igs::log_set_console_level(igs::LOG_DEBUG);
        assert_eq!(igs::log_console_level(), igs::LOG_DEBUG);
        igs::log_set_console_level(igs::LOG_INFO);
        assert_eq!(igs::log_console_level(), igs::LOG_INFO);
        igs::log_set_console_level(igs::LOG_WARN);
        assert_eq!(igs::log_console_level(), igs::LOG_WARN);
        igs::log_set_console_level(igs::LOG_ERROR);
        assert_eq!(igs::log_console_level(), igs::LOG_ERROR);
        igs::log_set_console_level(igs::LOG_FATAL);
        assert_eq!(igs::log_console_level(), igs::LOG_FATAL);
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
}
