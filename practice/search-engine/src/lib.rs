use wasm_bindgen::prelude::*;

pub trait SearchEngine {}

#[wasm_bindgen(start)]
fn run() {
    unsafe {
        alert("Hello, world!");
    }
}

#[wasm_bindgen]
unsafe extern "C" {
    unsafe fn alert(s: &str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target() {}
}
