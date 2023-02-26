mod antidbg;
mod exit_codes;

fn detect() {
    unsafe {
        antidbg::adbg_is_debugger_present();
        antidbg::adbg_being_debugged_peb();
        antidbg::adbg_nt_global_flag_peb();
        antidbg::adbg_check_remote_debugger_present();
        antidbg::adbg_check_window_class_name();
        antidbg::adbg_check_window_name();
        antidbg::adbg_process_file_name();
    }
}
