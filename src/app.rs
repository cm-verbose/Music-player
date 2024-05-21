use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

use crate::components::*;
use main_view::MainView; 
use error_view::ErrorView;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// The App component, the entry point of the app
#[component]
pub fn App() -> impl IntoView { 
  let (sidebar_state, set_sidebar_state) = create_signal(true);
  provide_context(sidebar_state);

  view! {
    <Router>
      <Routes>
        <Route path="/" view=move || view!{<MainView set_sidebar_state/> } />
        <Route path="/*any" view=ErrorView/>
      </Routes>
    </Router>
  }
}
