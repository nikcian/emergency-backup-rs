use std::{collections::HashMap, io::Write, path::PathBuf};

use eframe::egui::TextBuffer;

pub enum WorkingFolderError {
    WorkingFolderAlreadyExist,
    WorkingFolderCreationError,
    ConfigFileCreationError,
}

pub fn new_working_folder() -> Result<(), WorkingFolderError> {
    if check_working_folder() {
        return Err(WorkingFolderError::WorkingFolderAlreadyExist);
    }

    let path_dir = PathBuf::from("./emergency-backup-data");
    let path_log = PathBuf::from("./emergency-backup-data/logger.log");
    let path_conf = PathBuf::from("./emergency-backup-data/config.conf");

    if let Err(_) = std::fs::create_dir(path_dir) {
        return Err(WorkingFolderError::WorkingFolderCreationError);
    }

    if let Err(_) = std::fs::File::create(path_log) {
        return Err(WorkingFolderError::WorkingFolderCreationError);
    }

    if let Err(_) = std::fs::File::create(path_conf) {
        return Err(WorkingFolderError::WorkingFolderCreationError);
    }

    Ok(())
}

pub fn check_working_folder() -> bool {
    let path_log = PathBuf::from("./emergency-backup-data/logger.log");
    let path_conf = PathBuf::from("./emergency-backup-data/config.conf");

    if let Err(_) = std::fs::File::open(path_log) {
        return false;
    }

    if let Err(_) = std::fs::File::open(path_conf) {
        return false;
    }

    if get_config_opt().is_none() {
        return false;
    }

    true
}

pub fn create_config_file(
    selected_device: String,
    selected_path: Vec<String>,
) -> Result<(), WorkingFolderError> {
    let buf = String::from(
        "selected_device=".to_string()
            + selected_device.as_str()
            + "\nselected_path="
            + selected_path.join(";").as_str(),
    );
    let config_file = std::fs::File::create(PathBuf::from("./emergency-backup-data/config.conf"));

    if config_file.is_err() {
        return Err(WorkingFolderError::ConfigFileCreationError);
    }

    if std::fs::File::write(&mut config_file.unwrap(), buf.as_bytes()).is_err() {
        return Err(WorkingFolderError::ConfigFileCreationError);
    }

    Ok(())
}

pub fn get_config_opt() -> Option<(String, Vec<String>)> {
    let config_file = std::fs::File::open("./emergency-backup-data/config.conf");

    if config_file.is_err() {
        return None;
    }

    let mut buf = String::new();

    if std::io::Read::read_to_string(&mut config_file.unwrap(), &mut buf).is_err() {
        return None;
    }

    let map = buf
        .split('\n')
        .map(|opt| {
            let opt_vec = opt.split('=').collect::<Vec<&str>>();
            let mut vec = Vec::new();

            if opt_vec[0].eq("selected_path") {
                opt_vec[1]
                    .split(';')
                    .for_each(|path| vec.push(path.to_string()));
            } else if opt_vec[0].eq("selected_device") {
                vec.push(opt_vec[1].to_string());
            }

            (opt_vec[0].to_string(), vec)
        })
        .collect::<HashMap<String, Vec<String>>>();

    Some((
        map.get("selected_device").unwrap().clone()[0].take(),
        map.get("selected_path").unwrap().clone(),
    ))
}

pub fn _applescript(app_path: &str, folder_path: &str) -> String {
    let mut script_file = std::fs::File::create("./emergency-backup-data/script.scpt").unwrap();
    let app_name = "eb-rs-launcher.app";

    script_file
        .write(
            format!(
                "do shell script \"exec {} --autolaunch true {}\"",
                app_path, folder_path
            )
            .as_bytes(),
        )
        .unwrap();

    let mut script_builder = std::process::Command::new("osacompile")
        .args(&[
            "-o",
            &format!("{}/emergency-backup-data/{}", folder_path, app_name),
            &format!("{}/emergency-backup-data/script.scpt", folder_path),
        ])
        .spawn()
        .unwrap();

    script_builder.wait().unwrap();
    std::fs::remove_file(format!("{}/emergency-backup-data/script.scpt", folder_path)).unwrap();

    app_name.to_string()
}
