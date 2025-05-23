use crate::external_devices_management::external_devices_management::*;
use crate::mouse_pattern_recognition::mouse_pattern_recognition::*;
use crate::utility::utility::*;
use crate::working_folder_management::working_folder_management::*;

use fs_extra::dir::CopyOptions;
use std::io::Write;

#[cfg(target_os = "windows")]
use std::{os::windows::process::CommandExt, process::Command};

pub fn start_background_process(working_dir: &str) {
    let _ = std::env::set_current_dir(working_dir);

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let _ = std::process::Command::new(std::env::current_exe().unwrap())
            .args(&["--auto", "true"])
            .current_dir(working_dir)
            .stdout(std::fs::File::create("./emergency-backup-data/logger.log").unwrap())
            .stderr(std::fs::File::create("./emergency-backup-data/logger.log").unwrap())
            .spawn()
            .expect("Impossibile avviare il processo emergency-backup-rs.exe");
    }

    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let _ = Command::new("eb-rs")
            .creation_flags(CREATE_NO_WINDOW)
            .args(&["--auto", "true"])
            .current_dir(working_dir)
            .stdout(std::fs::File::create("./emergency-backup-data/logger.log").unwrap())
            .stderr(std::fs::File::create("./emergency-backup-data/logger.log").unwrap())
            .spawn()
            .expect("Impossibile avviare il processo emergency-backup-rs.exe");
    }
}

fn start_warning_process() -> bool {
    let mut warning_child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(&["--warning", "true"])
        .current_dir(std::env::current_dir().unwrap())
        .spawn()
        .expect("Impossibile avviare il processo emergency-backup-rs");

    let res = warning_child.wait().unwrap();

    if res.code().unwrap() == 0 {
        return true;
    }

    false
}

pub fn set_auto_launch() {
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        let app_name = "eb-rs";
        let app_path = std::env::current_exe().unwrap();
        let current_dir = std::env::current_dir().unwrap();
        let args = &[
            "--autolaunch",
            "true",
            current_dir.as_path().to_str().unwrap(),
        ];

        let auto = auto_launch::AutoLaunchBuilder::new()
            .set_app_name(&app_name)
            .set_app_path(app_path.to_str().unwrap())
            .set_use_launch_agent(false)
            .set_args(args)
            .build();

        let _ = auto.unwrap().enable().unwrap();
    }

    #[cfg(target_os = "macos")]
    {
        let working_folder = std::env::current_dir().unwrap();
        let app_name = _applescript(
            std::env::current_exe().unwrap().to_str().unwrap(),
            working_folder.to_str().unwrap(),
        );

        let auto = auto_launch::AutoLaunchBuilder::new()
            .set_app_name(&app_name)
            .set_app_path(&format!(
                "{}/emergency-backup-data/{}",
                working_folder.to_str().unwrap(),
                app_name
            ))
            .build();

        let _ = auto.unwrap().enable().unwrap();
    }
}

pub fn background_job() {
    println!("{} Background job started", chrono::prelude::Local::now());

    start_cpu_tracking();

    loop {
        if check_rectangle_pattern_with_duration(None)
            && check_rectangle_pattern_with_duration(None)
        {
            println!("{} Backup started", chrono::prelude::Local::now());

            let config_opt = get_config_opt();

            if config_opt.is_none() {
                break;
            }

            let (selected_device, selected_path) = config_opt.unwrap();
            let (src, dst);

            let confirm = start_warning_process();

            if !confirm {
                println!("{} Backup cancelled", chrono::prelude::Local::now());
                continue;
            }

            sound_first_confirm();

            let (timer, th) = create_countdown_timer(60, true);

            let mut res = false;

            while *timer.read().unwrap() > 0 && !res {
                let timer_clone = timer.clone();
                res = check_rectangle_pattern_with_duration(Some(timer_clone));
            }

            *timer.write().unwrap() = 0;
            let _ = th.join();

            if res == false {
                println!(
                    "{} Backup confirmation timeout",
                    chrono::prelude::Local::now()
                );
                continue;
            }

            #[cfg(target_os = "macos")]
            {
                (src, dst) = (
                    selected_path,
                    "/Volumes/".to_string() + selected_device.as_str(),
                );
            }

            #[cfg(target_os = "linux")]
            {
                (src, dst) = (
                    selected_path,
                    _get_device_path_from_name(selected_device).unwrap(),
                );
            }

            #[cfg(target_os = "windows")]
            {
                (src, dst) = (
                    selected_path,
                    _get_device_path_from_name(selected_device).unwrap()
                        + &format!(
                            "Backup_{}\\",
                            chrono::prelude::Local::now().format("%Y-%m-%d_%Hh-%Mm")
                        ),
                );

                let r = fs_extra::dir::create(dst.clone(), true);
                if r.is_err() {
                    println!(
                        "{} Error during acceding backup path: {:?}",
                        chrono::prelude::Local::now(),
                        r.err()
                    );
                    sound_alert();
                    continue;
                }
            }

            let instant = std::time::Instant::now();
            let mut log_file = std::fs::File::create("./backup.log").unwrap();

            match fs_extra::copy_items(
                src.as_slice(),
                dst.clone(),
                &CopyOptions::default().overwrite(true),
            ) {
                Ok(_) => {
                    println!("{} Backup successful", chrono::prelude::Local::now());

                    let time = instant.elapsed();
                    let size = src
                        .clone()
                        .into_iter()
                        .map(|src| fs_extra::dir::get_size(src).unwrap())
                        .sum::<u64>();

                    log_file
                        .write(
                            format!("{} Backup successuful!\n\n", chrono::prelude::Local::now())
                                .as_bytes(),
                        )
                        .unwrap();
                    log_file
                        .write(format!("Backup size: {} bytes\n", size).as_bytes())
                        .unwrap();
                    log_file
                        .write(format!("Time elapsed: {:?}\n", time).as_bytes())
                        .unwrap();

                    sound_success();

                    match fs_extra::move_items(
                        &["./backup.log"],
                        dst.clone(),
                        &CopyOptions::default().overwrite(true),
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            println!(
                                "{} Error during backup logfile creation: {:?}",
                                chrono::prelude::Local::now(),
                                err
                            );
                            sound_alert();
                        }
                    }
                }
                Err(err) => {
                    println!(
                        "{} Backup failed, error during backup: {:?}",
                        chrono::prelude::Local::now(),
                        err
                    );
                    log_file
                        .write(
                            format!(
                                "{} Backup failed, error during backup: {:?}\n",
                                chrono::prelude::Local::now(),
                                err
                            )
                            .as_bytes(),
                        )
                        .unwrap();

                    match fs_extra::move_items(
                        &["./backup.log"],
                        dst.clone(),
                        &CopyOptions::default().overwrite(true),
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            println!(
                                "{} Error during backup logfile creation: {:?}",
                                chrono::prelude::Local::now(),
                                err
                            );
                        }
                    }
                    sound_alert();
                }
            }
        }
    }
}
