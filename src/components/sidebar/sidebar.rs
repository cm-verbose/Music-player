use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
  let sidebar_state = use_context::<ReadSignal<bool>>(); 
  view! {
    <div class="Sidebar" data-collapsed=move || {
        if let Some(state) = sidebar_state {
          if state.get(){
            "visible"
          } else {
            "collapsed"
          }
        } else {
          "collapsed"
        }
    }>
      <ul>
        <li><a href="/">Library</a></li>
      </ul>
    </div>
  }
}
