use embedded_game_of_life::Plane;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::*;

use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Game of Life for Embedded", &output_settings);

    let mut plane = Plane::<65536>::from_magnification(160, 120, 2).unwrap();
    plane.randomize(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    plane.draw(&mut display).unwrap();

    let mut stop = false;
    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::MouseButtonUp { .. } => {
                    stop = !stop;
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
            thread::sleep(Duration::from_millis(200));
        }

        //thread::sleep(Duration::from_millis(200));
        if !stop {
            if plane.tick() {
                plane.randomize(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                );
            }
            plane.draw(&mut display).unwrap();
        }
    }
}
