use leptos::*;
use leptos_router::*; 
use wasm_bindgen::prelude::*;

use crate::components::*;
use main_view::MainView;
use sidebar::Sidebar;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
  let (sidebar_state, set_sidebar_state) = create_signal(true);
  provide_context(sidebar_state);
  view! {
    <Router>
      <div class="App">
        <Sidebar/>
        <MainView set_sidebar_state/>
      </div>
    </Router>
  }
}
