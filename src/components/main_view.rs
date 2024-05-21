use leptos::*;

use crate::components::*;
use library_view::LibraryView;
use sidebar::Sidebar;

#[derive(Clone)]
pub struct PlaylistItem {
  pub name: String,
  pub id: u32,
}

#[component]
pub fn MainView(set_sidebar_state: WriteSignal<bool>) -> impl IntoView {
  let (playlist_items, set_playlist_items) = create_signal::<Vec<PlaylistItem>>(vec![]);
  provide_context(playlist_items);

  view! {
    <div class="App">
      <Sidebar/>
      <LibraryView set_sidebar_state set_playlist_items/>
    </div>
  }
}
