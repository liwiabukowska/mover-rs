use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, SetCursorPos};
use winapi::um::winuser::{keybd_event, VK_LSHIFT, KEYEVENTF_KEYUP};

use std::f64::consts::PI;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    const INITIAL_SLEEP_SECS: u64 = 5;
    const FIGURE_SLEEP_MS: u64 = 15;
    const BETWEEN_SLEEP_SECS: u64 = 120;
    // const CIRCLE_RADIUS: i32 = 50;
    const ELLIPSE_WIDTH: i32 = 100;
    const ELLIPSE_HEIGHT: i32 = 50;
    const STEPS: usize = 30;
        
    sleep(Duration::from_secs(INITIAL_SLEEP_SECS));

    let builder = FigureBuilder::new(Duration::from_millis(FIGURE_SLEEP_MS));
    
    // let circle = builder.build_circle(CIRCLE_RADIUS, STEPS);    
    let ellipse = builder.build_ellipse(ELLIPSE_WIDTH, ELLIPSE_HEIGHT, STEPS);

    loop {
        let (mouse_x, mouse_y) = get_mouse_position();
        ellipse.draw(mouse_x, mouse_y);

        sleep(Duration::from_secs(BETWEEN_SLEEP_SECS));
    }
}

pub struct Figure {
    offsets: Vec<(i32, i32)>,
    sleep_time: Duration,
}

impl Figure {
    fn draw(&self, start_x: i32, start_y: i32) {
        for &(dx, dy) in &self.offsets {
            let x = start_x + dx;
            let y = start_y + dy;
            set_mouse_position((x, y));
            bump_wake_time();
            sleep(self.sleep_time);
        }
        set_mouse_position((start_x, start_y));
    }
}

pub struct FigureBuilder {
    sleep_time: Duration,
}

impl FigureBuilder {
    pub fn new(sleep_time: Duration) -> Self {
        FigureBuilder { sleep_time }
    }

    pub fn build_circle(&self, radius: i32, steps: usize) -> Figure {
        let offsets = (0..steps)
            .map(|i| {
                let theta = 2.0 * PI * (i as f64) / (steps as f64);
                let x = (radius as f64 * theta.cos()).round() as i32;
                let y = (radius as f64 * theta.sin()).round() as i32;
                (x, y)
            })
            .collect();

        Figure {
            offsets,
            sleep_time: self.sleep_time,
        }
    }

    pub fn build_ellipse(&self, width: i32, height: i32, steps: usize) -> Figure {
        let offsets: Vec<(i32, i32)> = (0..steps)
            .map(|i| {
                let theta = 2.0 * PI * (i as f64) / (steps as f64);
                let x = (width as f64 * theta.cos()).round() as i32;
                let y = (height as f64 * theta.sin()).round() as i32;
                (x, y)
            })
            .collect();

        Figure {
            offsets,
            sleep_time: self.sleep_time,
        }
    }
}

fn get_mouse_position() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point);
    }
    let (x, y) = (point.x, point.y);
    // println!("get: x={x} y={y}");
    (x, y)
}

fn set_mouse_position((x, y): (i32, i32)) {
    unsafe {
        SetCursorPos(x, y);
    }
    // println!("set: x={x} y={y}");
}

fn bump_wake_time() {
    unsafe {
        keybd_event(VK_LSHIFT as u8, 0, 0, 0);
        keybd_event(VK_LSHIFT as u8, 0, KEYEVENTF_KEYUP, 0);
    }
}
