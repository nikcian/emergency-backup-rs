#![windows_subsystem = "windows"]

use background_process_management::background_process_management::*;
use gui::gui::*;

mod gui {
    pub mod gui;
}

mod background_process_management {
    pub mod background_process_management;
}

mod external_devices_management {
    pub mod external_devices_management;
}

mod mouse_pattern_recognition {
    pub mod mouse_pattern_recognition;
}

mod working_folder_management {
    pub mod working_folder_management;
}

mod utility {
    pub mod utility;
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() >= 3 && args[1].eq("--auto") && args[2].eq("true") {
        background_job();
    } else if args.len() >= 4 && args[1].eq("--autolaunch") && args[2].eq("true") {
        start_background_process(&args[3]);
    } else if args.len() >= 3 && args[1].eq("--warning") && args[2].eq("true") {
        start_warning_window();
    } else {
        start_config_window();
    }
}
