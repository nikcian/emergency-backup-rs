use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use display_info::DisplayInfo;
use mouse_rs::Mouse;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

pub fn check_rectangle_pattern_with_duration(duration: Option<Arc<RwLock<i8>>>) -> bool {
    let mouse = Mouse::new();
    let current_display = DisplayInfo::all()
        .unwrap()
        .into_iter()
        .filter(|display| display.is_primary)
        .collect::<Vec<DisplayInfo>>()
        .pop()
        .unwrap();

    let max_x;
    let max_y;

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        max_x = (current_display.width as f32) * (current_display.scale_factor) - 1 as f32;
        max_y = (current_display.height as f32) * (current_display.scale_factor) - 1 as f32;
    }

    #[cfg(target_os = "macos")]
    {
        max_x = (current_display.width as f32) - 1.0;
        max_y = (current_display.height as f32) - 1.0;
    }

    let mut corner_points = Vec::from([
        Point::new(0.0, 0.0),
        Point::new(0.0, max_y),
        Point::new(max_x, 0.0),
        Point::new(max_x, max_y),
    ]);

    let mut i;
    let mut current_point;

    loop {
        if duration.as_ref().is_some_and(|d| *d.read().unwrap() <= 0) {
            return false;
        }

        std::thread::sleep(Duration::from_millis(50));

        if duration.as_ref().is_some_and(|d| *d.read().unwrap() <= 0) {
            return false;
        }

        let current_pos = mouse
            .get_position()
            .map(|p| Point::new(p.x as f32, p.y as f32))
            .unwrap();

        if corner_points.contains(&current_pos) {
            (i, _) = corner_points
                .iter()
                .enumerate()
                .find(|(_, p)| **p == current_pos)
                .unwrap();
            current_point = corner_points.remove(i);
            corner_points.push(current_point.clone());
            break;
        }
    }

    let starting_point = current_point.clone();

    while !corner_points.is_empty() {
        if duration.as_ref().is_some_and(|d| *d.read().unwrap() <= 0) {
            return false;
        }

        std::thread::sleep(Duration::from_millis(50));

        if duration.as_ref().is_some_and(|d| *d.read().unwrap() <= 0) {
            return false;
        }

        let current_pos = mouse
            .get_position()
            .map(|p| Point::new(p.x as f32, p.y as f32))
            .unwrap();

        if current_pos.x != current_point.x && current_pos.y != current_point.y {
            break;
        }

        if corner_points.contains(&current_pos) {
            (i, _) = corner_points
                .iter()
                .enumerate()
                .find(|(_, p)| **p == current_pos)
                .unwrap();
            current_point = corner_points.remove(i);

            if corner_points.len() == 0 && current_point != starting_point {
                corner_points.push(starting_point.clone());
            }
        }
    }

    if corner_points.is_empty() {
        return true;
    } else {
        return false;
    }
}
