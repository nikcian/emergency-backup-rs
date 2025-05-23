use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn create_countdown_timer(seconds: i8, with_sound: bool) -> (Arc<RwLock<i8>>, JoinHandle<()>) {
    let time_left = Arc::new(RwLock::new(seconds));
    let time_left_clone = time_left.clone();

    let note_duration = std::time::Duration::from_millis(250);

    let th = std::thread::spawn(move || loop {
        if with_sound {
            std::thread::sleep(Duration::from_secs(1) - note_duration);
            sound_hint(note_duration);
        } else {
            std::thread::sleep(Duration::from_secs(1));
        }

        *time_left_clone.write().unwrap() -= 1;

        if *time_left_clone.read().unwrap() <= 0 {
            break;
        }
    });

    (time_left, th)
}

pub fn start_cpu_tracking() {
    let _ = std::thread::spawn(|| {
        let mut sys = sysinfo::System::new_all();
        let stat_time = std::time::Duration::from_millis(400);
        let update_time = std::time::Duration::from_secs(120);

        sys.refresh_all();
        sys.refresh_all();

        loop {
            std::thread::sleep(update_time - stat_time);
            sys.refresh_all();

            std::thread::sleep(stat_time);
            println!(
                "{} Process usage: {}%",
                chrono::prelude::Local::now(),
                sys.process(sysinfo::Pid::from_u32(std::process::id()))
                    .unwrap()
                    .cpu_usage()
                    / sys.cpus().len() as f32
            );
        }
    });
}

pub fn kill_old_background_job() {
    #[cfg(windows)]
    {
        let this_pid = sysinfo::get_current_pid().unwrap();
        let s = sysinfo::System::new_all();

        for process in s.processes_by_name("eb-rs.exe") {
            if process.pid() != this_pid {
                std::process::Command::new("taskkill")
                    .args(&["/PID", &process.pid().to_string(), "/F"])
                    .output()
                    .ok();
            }
        }
    }
    #[cfg(unix)]
    {
        let this_pid = sysinfo::get_current_pid().unwrap();
        let s = sysinfo::System::new_all();
        #[cfg(target_os = "linux")]
        let mut counter = 0;

        for process in s.processes_by_exact_name("eb-rs") {
            let p_pid = process.pid();
            if p_pid != this_pid {
                #[cfg(target_os = "linux")]
                {
                    counter += 1;

                    if counter < 10 {
                        continue;
                    }
                }

                std::process::Command::new("kill")
                    .args(&["-15", &p_pid.as_u32().to_string()])
                    .output()
                    .ok();
            }
        }
    }
}

pub fn sound_hint(duration: std::time::Duration) {
    const LA4: f32 = 440.0;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(SineWave::new(LA4).take_duration(duration).amplify(0.2));
    sink.sleep_until_end();
}

pub fn sound_first_confirm() {
    const DOD5: f32 = 554.37;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(
        SineWave::new(0.0)
            .take_duration(std::time::Duration::from_millis(125))
            .amplify(0.0),
    );
    sink.append(
        SineWave::new(DOD5)
            .take_duration(std::time::Duration::from_millis(125))
            .amplify(0.2),
    );
    sink.append(
        SineWave::new(0.0)
            .take_duration(std::time::Duration::from_millis(5))
            .amplify(0.0),
    );
    sink.append(
        SineWave::new(DOD5)
            .take_duration(std::time::Duration::from_millis(125))
            .amplify(0.2),
    );
    sink.append(
        SineWave::new(0.0)
            .take_duration(std::time::Duration::from_millis(5))
            .amplify(0.0),
    );
    sink.append(
        SineWave::new(DOD5)
            .take_duration(std::time::Duration::from_millis(125))
            .amplify(0.2),
    );
    sink.append(
        SineWave::new(0.0)
            .take_duration(std::time::Duration::from_millis(125))
            .amplify(0.0),
    );
    sink.sleep_until_end();
}

pub fn sound_success() {
    const MI5: f32 = 659.26;
    const SOLB5: f32 = 830.61;
    const SI6: f32 = 1975.53;
    const SOLB6: f32 = 1661.22;
    const MI7: f32 = 2637.02;

    const BASS_AMPL: f32 = 0.15;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(
        SineWave::new(MI5)
            .take_duration(std::time::Duration::from_millis(500))
            .amplify(BASS_AMPL)
            .mix(SineWave::new(SI6))
            .take_duration(std::time::Duration::from_millis(500))
            .amplify(0.2),
    );
    sink.append(
        SineWave::new(MI5)
            .take_duration(std::time::Duration::from_millis(500))
            .amplify(BASS_AMPL)
            .mix(SineWave::new(SI6))
            .take_duration(std::time::Duration::from_millis(500))
            .amplify(0.2),
    );
    sink.append(
        SineWave::new(MI5)
            .take_duration(std::time::Duration::from_millis(250))
            .amplify(BASS_AMPL)
            .mix(SineWave::new(SI6))
            .take_duration(std::time::Duration::from_millis(250))
            .amplify(0.2),
    );

    sink.append(
        SineWave::new(SOLB5)
            .take_duration(std::time::Duration::from_millis(250))
            .amplify(BASS_AMPL)
            .mix(SineWave::new(SOLB6))
            .take_duration(std::time::Duration::from_millis(250))
            .amplify(0.2),
    );
    sink.append(
        SineWave::new(MI5)
            .take_duration(std::time::Duration::from_millis(500))
            .amplify(BASS_AMPL)
            .mix(SineWave::new(MI7))
            .take_duration(std::time::Duration::from_millis(500 + 63))
            .amplify(0.2),
    );
    sink.sleep_until_end();
}

pub fn sound_alert() {
    const DOD5: f32 = 554.37;
    const MI5: f32 = 659.26;
    const SOL5: f32 = 783.99;
    const LAD5: f32 = 932.33;

    const DOD6: f32 = 1108.73;
    const MI6: f32 = 1318.51;
    const SOL6: f32 = 1567.98;
    const LAD6: f32 = 1864.66;

    let duration = std::time::Duration::from_millis(125);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(SineWave::new(DOD5).amplify(0.08).take_duration(duration));
    sink.append(SineWave::new(MI5).amplify(0.08).take_duration(duration));
    sink.append(SineWave::new(SOL5).amplify(0.08).take_duration(duration));
    sink.append(SineWave::new(LAD5).amplify(0.08).take_duration(duration));

    sink.append(SineWave::new(DOD6).amplify(0.2).take_duration(duration));
    sink.append(SineWave::new(MI6).amplify(0.2).take_duration(duration));
    sink.append(SineWave::new(SOL6).amplify(0.2).take_duration(duration));
    sink.append(SineWave::new(LAD6).amplify(0.2).take_duration(duration));

    sink.sleep_until_end();
}
