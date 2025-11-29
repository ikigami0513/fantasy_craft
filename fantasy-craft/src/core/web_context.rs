use macroquad::prelude::*;

#[allow(dead_code)]
unsafe extern "C" {
    fn js_get_base_url(ptr: *mut u8, cap: i32) -> i32;
}

pub struct WebContext;

impl WebContext {
    /// Retrieves the base URL for assets from the JavaScript layer.
    pub fn get_base_url() -> String {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            // 1. Allocate a buffer in Rust memory to hold the string
            let mut buffer = [0u8; 1024]; // 1024 chars should be enough for a URL
            
            // 2. Call the JS function, passing our buffer's pointer and capacity
            let len = js_get_base_url(buffer.as_mut_ptr(), buffer.len() as i32);
            
            // 3. Convert the written bytes back to a Rust String
            let bytes = &buffer[0..len as usize];
            String::from_utf8_lossy(bytes).to_string()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Fallback for desktop builds (local debugging)
            String::from("")
        }
    }
}
