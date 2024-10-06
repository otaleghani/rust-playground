use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub async fn fetch_data(url: String) -> Result<JsValue, JsValue> {
    let window = web_sys::window().ok_or("nope")?;
    let response_promise = window.fetch_with_str(&url);

    let response_value = JsFuture::from(response_promise).await?;
    let response: Response = response_value.dyn_into()?;

    let json_promise = response.json()?;
    let json = JsFuture::from(json_promise).await?;

    Ok(json)
}
