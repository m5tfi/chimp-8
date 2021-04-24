#![warn(clippy::pedantic, clippy::all)]

use chimp_core::Vm;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
pub struct VmWasm {
    vm: Vm,
    ctx: CanvasRenderingContext2d,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl VmWasm {
    #[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<VmWasm, JsValue> {
        let vm = Vm::default();
        let document: Document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Ok(VmWasm { vm, ctx })
    }

    #[wasm_bindgen]
    pub fn load_game(&mut self, data: &Uint8Array) {
        self.vm.load_program(&data.to_vec());
    }

    #[allow(clippy::cast_precision_loss)]
    #[wasm_bindgen]
    pub fn draw(&mut self, scale: usize) {
        let pixels = self.vm.get_display();
        for (i, px) in pixels.iter().enumerate() {
            if *px {
                let x = i % Vm::SCREEN_WIDTH;
                let y = i / Vm::SCREEN_WIDTH;
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64,
                );
            }
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.vm.tick();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.vm.tick_timers();
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.vm.reset();
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, event: &KeyboardEvent, pressed: bool) {
        let keycode = event.key();
        if let Some(key) = Self::key_to_hex(&keycode) {
            self.vm.keypress(key, pressed)
        }
    }

    fn key_to_hex(key: &str) -> Option<usize> {
        match key {
            "1" => Some(0x1),
            "2" => Some(0x2),
            "3" => Some(0x3),
            "4" => Some(0xC),

            "q" => Some(0x4),
            "w" => Some(0x5),
            "e" => Some(0x6),
            "r" => Some(0xD),

            "a" => Some(0x7),
            "s" => Some(0x8),
            "d" => Some(0x9),
            "f" => Some(0xE),

            "z" => Some(0xA),
            "x" => Some(0x0),
            "c" => Some(0xB),
            "v" => Some(0xF),

            _ => None,
        }
    }
}
