# debugprotector

## Using default method

Default method will check debugger and self kill the program

```
use debugprotector::detect;

fn main() {
  detect();
}
```

## Using single method.

This crate has the following functions:

```
use debugprotector::antidbg;

antidbg::adbg_is_debugger_present();
antidbg::adbg_being_debugged_peb();
antidbg::adbg_nt_global_flag_peb();
antidbg::adbg_check_remote_debugger_present();
antidbg::adbg_check_window_class_name();
antidbg::adbg_check_window_name();
antidbg::adbg_process_file_name();
```
