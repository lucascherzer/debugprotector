mod antidbg;
mod exit_codes;

use crate::exit_codes::DebugStatus;

// Check for a debugger and exit if there is one
pub fn detect() {
    if debugger_present() {
        std::process::exit(0); // Exit code would need to be changed accordingly
    }
}

/// Checks whether there is a debugger present
/// # Returns
/// * `true` if it found a debugger
/// * `false` if it did **not** find a debugger
pub fn debugger_present() -> bool {
    let checks: [unsafe fn() -> DebugStatus; 7] = [
            antidbg::adbg_is_debugger_present,
            antidbg::adbg_being_debugged_peb,
            antidbg::adbg_nt_global_flag_peb,
            antidbg::adbg_check_remote_debugger_present,
            antidbg::adbg_check_window_class_name,
            antidbg::adbg_check_window_name,
            antidbg::adbg_process_file_name
        ];
        unsafe {
        for check in checks {
            if let DebugStatus::None = check() {
                continue
            } else {
                return true
            }
        }
        false
    }
}
