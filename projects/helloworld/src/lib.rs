use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator when the feature is enabled
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Contract {
    value: String,
}

#[wasm_bindgen]
impl Contract {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            value: String::from("Hello, world!"),
        }
    }

    pub fn set_greeting(&mut self, greeting: String) {
        self.value = greeting;
    }

    pub fn get_greeting(&self) -> String {
        self.value.clone()
    }

    pub fn reset_to_default(&mut self) {
        self.value = String::from("Hello, world!");
    }
}

// Required for proper WASM instantiation
#[wasm_bindgen(start)]
pub fn start() {
    // Empty start function, but we keep it for proper WASM instantiation
} 