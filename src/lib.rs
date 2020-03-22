use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let cancas = document.get_element_by_id("canvas")?;
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    context.begin_path();

    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();
    
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)unwrap();

    context.move_to(65.0, 65.0);
}

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

