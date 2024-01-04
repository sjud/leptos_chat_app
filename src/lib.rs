use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {}
}

cfg_if::cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            #[cfg(debug_assertions)]
            console_error_panic_hook::set_once();
            leptos::mount_to_body(App);
        }
    }
}
