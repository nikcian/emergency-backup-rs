use crate::working_folder_management::working_folder_management::*;

pub fn get_external_devices() -> Vec<String> {
    let disk_list = sysinfo::Disks::new_with_refreshed_list();

    disk_list
        .into_iter()
        .filter(|d| d.is_removable())
        .map(|d| d.name().to_str().unwrap().to_string())
        .collect::<Vec<String>>()
}

pub fn _is_selected_device_attached() -> bool {
    let config_opt = get_config_opt();

    if config_opt.is_none() {
        return false;
    }

    let (selected_device, _) = config_opt.unwrap();

    get_external_devices()
        .into_iter()
        .find(|d| *d == selected_device)
        .is_some()
}

pub fn _get_device_path_from_name(name: String) -> Option<String> {
    let disk_list = sysinfo::Disks::new_with_refreshed_list();

    disk_list
        .into_iter()
        .find(|d| d.is_removable() && d.name().to_str().unwrap().to_string() == name)
        .map(|d| d.mount_point().to_str().unwrap().to_string())
}
