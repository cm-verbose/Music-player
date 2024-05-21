use leptos::*;

/// Represents the sidebar
#[component]
pub fn Sidebar() -> impl IntoView {
  let sidebar_state = if let Some(state) = use_context::<ReadSignal<bool>>(){
    state
  } else {
    panic!("The sidebar_state context value was not provided");
  }; 

  view! {
    <div class="Sidebar" data-collapsed=move || {
        if sidebar_state.get(){ "visible" } else { "collapsed" }
      }>
      <ul>
        <li><a href="/">Library</a></li>
      </ul>
    </div>
  }
}
