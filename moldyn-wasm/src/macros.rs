macro_rules! TYPESCRIPT {
    (
        $s:literal
    ) => {
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r"\n" + $s + r"\n";
    };
}
