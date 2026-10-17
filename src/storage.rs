//! Small text values kept in `<key>.txt` natively and in localStorage in the browser.

#[cfg(not(target_arch = "wasm32"))]
pub fn read(key: &str) -> String {
    std::fs::read_to_string(format!("{key}.txt")).unwrap_or_default()
}

/// A value that cannot be saved is not worth crashing over.
#[cfg(not(target_arch = "wasm32"))]
pub fn write(key: &str, text: &str) {
    std::fs::write(format!("{key}.txt"), text).ok();
}

// Provided by web/storage.js.
#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn storage_read(key: *const u8, key_length: u32, buffer: *mut u8, capacity: u32) -> u32;
    fn storage_write(key: *const u8, key_length: u32, text: *const u8, length: u32);
}

/// Lets the JS bundle check that web/storage.js matches this build.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
extern "C" fn storage_crate_version() -> u32 {
    1
}

#[cfg(target_arch = "wasm32")]
pub fn read(key: &str) -> String {
    let mut buffer = vec![0; 4096];
    let length = unsafe {
        storage_read(
            key.as_ptr(),
            key.len() as u32,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
        )
    };
    buffer.truncate(length as usize);
    String::from_utf8(buffer).unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub fn write(key: &str, text: &str) {
    unsafe {
        storage_write(
            key.as_ptr(),
            key.len() as u32,
            text.as_ptr(),
            text.len() as u32,
        )
    }
}
