use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
 }

#[wasm_bindgen]
pub fn process_message(msg: &str) -> String {
    format!("Welcome {}", &msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_message() {
        let result: String = process_message("human");
        assert_eq!("Welcome human", result);
    }
}

