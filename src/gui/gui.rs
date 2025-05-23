use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use eframe::egui;

use crate::background_process_management::background_process_management::*;
use crate::external_devices_management::external_devices_management::*;
use crate::utility::utility::*;
use crate::working_folder_management::working_folder_management::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum WindowStatus {
    #[default]
    ConfigWindow,
    WarningWindow,
}

pub fn start_config_window() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true)
            .with_resizable(false)
            .with_inner_size([640.0, 240.0])
            .with_maximize_button(false)
            .with_drag_and_drop(false),
        follow_system_theme: true,
        vsync: false,
        run_and_return: true,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "emergency-backup-rs",
        options,
        Box::new(|_cc| Ok(Box::new(App::new(_cc, WindowStatus::ConfigWindow, None)))),
    );
}

pub fn start_warning_window() -> bool {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true)
            .with_resizable(false)
            .with_inner_size([640.0, 240.0])
            .with_drag_and_drop(false)
            .with_close_button(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_always_on_top(),
        follow_system_theme: true,
        vsync: false,
        run_and_return: true,
        ..Default::default()
    };

    let (time_left, th1) = create_countdown_timer(15, true);

    let time_left_clone = time_left.clone();
    let check_result = time_left.clone();

    let th2 = std::thread::spawn(move || loop {
        if *time_left_clone.read().unwrap() <= 0 {
            std::process::exit(0);
        }
    });

    let _ = eframe::run_native(
        "emergency-backup-rs",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(App::new(
                _cc,
                WindowStatus::WarningWindow,
                Some(time_left),
            )))
        }),
    );

    th1.join().unwrap();
    th2.join().unwrap();

    if *check_result.read().unwrap() <= 0 {
        return false;
    }

    true
}

struct App {
    status: WindowStatus,
    picked_path: Vec<(String, bool)>,
    picked_device: Option<String>,
    time_left: Option<Arc<RwLock<i8>>>,
}

impl App {
    fn new(
        _cc: &eframe::CreationContext,
        status: WindowStatus,
        time_left: Option<Arc<RwLock<i8>>>,
    ) -> Self {
        let config_opt = get_config_opt();

        let mut picked_path = Vec::new();
        let mut picked_device = None;

        if let Some((old_picked_device, old_picked_path)) = config_opt {
            picked_path = old_picked_path
                .iter()
                .map(|path| (path.clone(), false))
                .collect();
            picked_device = Some(old_picked_device);
        }

        App {
            status,
            picked_path,
            picked_device,
            time_left,
        }
    }

    fn config_window(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Choose up to five directories to save in case of emergency!");

            if self.picked_path.len() < 5 && ui.button("Open directory…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let p = path.to_str().unwrap().to_string();
                    if !self.picked_path.contains(&(p.clone(), false)) {
                        self.picked_path.push((p, false));
                    }
                }
            }

            for i in 0..self.picked_path.len() {
                ui.horizontal(|ui| {
                    if ui.button("-").clicked() {
                        self.picked_path[i].1 = true;
                    }

                    ui.label(format!("{:?}", self.picked_path[i].0));
                });
            }

            self.picked_path.retain(|(_, flag)| !flag);

            ui.add_space(20.0);
            ui.heading("Choose an external device to use in case of emergency!");

            egui::ComboBox::new("select-menu", "").show_ui(ui, |ui| {
                for option in get_external_devices() {
                    ui.selectable_value(
                        &mut self.picked_device,
                        Some(option.clone()),
                        option.to_string(),
                    );
                }
            });

            if let Some(picked_device) = &self.picked_device {
                ui.horizontal(|ui| {
                    ui.label("Picked external device:");
                    ui.monospace(picked_device);
                });
            }
        });

        egui::TopBottomPanel::bottom("bottom-panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                if ui.button("Start emergency backup!").clicked() {
                    if self.picked_path.len() > 0 && self.picked_device.is_some() {
                        kill_old_background_job();

                        let _ = new_working_folder();
                        let _ = create_config_file(
                            self.picked_device.take().unwrap(),
                            self.picked_path
                                .clone()
                                .into_iter()
                                .map(|(path, _)| path)
                                .collect::<Vec<String>>(),
                        );

                        set_auto_launch();

                        start_background_process(
                            &std::env::current_dir().unwrap().to_str().unwrap(),
                        );

                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            });
    }

    fn warning_window(&mut self, ctx: &egui::Context) {
        let time = *self.time_left.clone().unwrap().read().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Warning!");
            ui.label("The backup process has started, if you want to cancel it press the button, otherwise wait and make the signal again!");
            ui.label(format!("The window will close automatically in {:?} seconds.", time));
            ctx.request_repaint_after(Duration::from_millis(50));
        });

        if time == 0 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::TopBottomPanel::bottom("bottom-panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                if ui.button("Cancel").clicked() {
                    std::process::exit(1);
                }
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.status {
            WindowStatus::ConfigWindow => self.config_window(ctx),
            WindowStatus::WarningWindow => self.warning_window(ctx),
        }
    }
}
