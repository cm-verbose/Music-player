use leptos::*;
use leptos_router::*;

#[component]
pub fn ErrorView() -> impl IntoView {
  let router_context = use_router();
  let path: Memo<String> = router_context.pathname();
  view! {
    <div class="ErrorView">
      <h1>"We couldn't find this page"</h1>
      <p>{move || path.get()}</p>
    </div>
  }
}
