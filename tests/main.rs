#[cfg(test)]
mod tests {
    use tabler_dominator::icon;
    use dominator::{html, with_node, Dom};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);


    #[wasm_bindgen_test]
    fn it_works() {
        let ic: Dom = icon!("ad-2");
        web_sys::console::log_1(&format!("{:?}", ic).into());
    }
}
