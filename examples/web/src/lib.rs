use embedded_graphics_web_simulator::display::WebSimulatorDisplay;
use embedded_graphics_web_simulator::output_settings::OutputSettingsBuilder;
use js_sys::*;
use wasm_bindgen::prelude::*;

use embedded_game_of_life::Plane;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct LifeOfGame {
    plane: Box<Plane<65536>>,
    display: Box<WebSimulatorDisplay>,
}

#[wasm_bindgen]
impl LifeOfGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> LifeOfGame {
        let output_settings = OutputSettingsBuilder::new().scale(3).build();
        let display = WebSimulatorDisplay::new((320, 240), &output_settings);
        let mut plane = Plane::<65536>::from_magnification(160, 120, 2).unwrap();
        let seed = u64::from_be_bytes(Date::now().to_be_bytes());
        plane.randomize(seed);

        Self {
            plane: Box::new(plane),
            display: Box::new(display),
        }
    }
    #[wasm_bindgen]
    pub fn tick(&mut self) {
        if self.plane.tick() {
            let seed = u64::from_be_bytes(Date::now().to_be_bytes());
            self.plane.randomize(seed);
        }
        self.plane.draw(self.display.as_mut()).unwrap();
    }
}
