use log::{Level};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;



// Debug build:     wasm-pack build --target web --dev -- --features debug-logging
// Release build:   wasm-pack build --target web
// Launch server:   python3 -m http.server 8000
// Address:		    http://localhost:8000/



#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
	
	initialize_logging_in_debug()?;
	
	return Ok(());
}

fn initialize_logging_in_debug() -> Result<(), JsValue> {
	
	#[cfg(feature = "debug-logging")] {
		std::panic::set_hook(Box::new(console_error_panic_hook::hook));
		console_log::init_with_level(Level::Warn).expect("Couldn't initialize logger");
		console::log_1(&"Panic hook and logger initialized.".into());
	}
	
	return Ok(());
}