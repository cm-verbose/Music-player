use leptos::{html::Input, *};
use web_sys::MouseEvent;
mod playlist;

#[derive(Clone)]
struct PlaylistItem {
  name: String,
  id: u32,
}

/// Represents the main component presenting the playlists and the songs
#[component]
pub fn MainView(set_sidebar_state: WriteSignal<bool>) -> impl IntoView {
  let (playlist_items, set_playlist_items) = create_signal::<Vec<PlaylistItem>>(vec![]);
  let (playlist_dialog_state, set_playlist_dialog_state) = create_signal(false);
  let playlist_title_noderef = NodeRef::<Input>::new();
  let sidebar_state: ReadSignal<bool> = {
    if let Some(state) = use_context::<ReadSignal<bool>>() {
      state
    } else {
      panic!("The sidebar_state state attribute was not defined")
    }
  };

  let mut counter: u32 = 0; // FIXME: This is bad lol
  const MAX_PLAYLIST_TITLE_LENGTH: usize = 256;
  view! {
    <main class="Main" data-view-state=move || {
        if sidebar_state.get() {
          "compressed"
        } else {
          "full"
        }
      }>
      <div id="top">
        <header>
          <button id="sidebar-toggle-button" on:click=move |_:MouseEvent| {
            set_sidebar_state.update(|current_state| *current_state = !(*current_state));
          }>
          <img alt="toggle navigation state" src=move || {
            let panelName = |icon: &str| format!("./public/svg/{}_panel_left.svg", icon);
            panelName(if sidebar_state.get(){
              "close"
            } else {
              "open"
            })
          }/>
        </button>
        </header>
        <hgroup>
          <h1>"Library"</h1>
          <button id="playlist-create-button" on:click=move|_| {
            set_playlist_dialog_state.update(|state| *state = true)
          }>
            "Create playlist"
          </button>
          <hr/>
        </hgroup>
      </div>
      <div id="playlist-container">
        <Show when=move|| playlist_items.get().is_empty()>
          <div id="empty-playlist-placeholder">"No playlists found"</div>
        </Show>
        <For
          each=move|| playlist_items.get()
          key=move|playlist_items| playlist_items.id
          children=move |item:PlaylistItem| {
            let PlaylistComponent = playlist::Playlist;
            let name = item.name; 
            view!{
              <PlaylistComponent name={name}/>
            }
          }
        />
      </div>
    </main>
    <div id="playlist-create-dialog" data-visible=move|| {
      if playlist_dialog_state.get(){
        "visible"
      } else{
        "hidden"
      }
    }>
      <div id="dialog-closer" on:click=move|_| {
        set_playlist_dialog_state.update(|state| *state = false)
      }></div>
      <div id="dialog">
        <h2>"Create new playlist"</h2>
        <input type="text" placeholder="Playlist name" id="playlist-name-input" spellcheck="false"
          autocomplete="off" maxlength={MAX_PLAYLIST_TITLE_LENGTH} _ref={playlist_title_noderef}
        />
        <button id="create-playlist-button" on:click= move|_| {
          if let Some(input) = playlist_title_noderef.get() {
            let user_title = input.value();
            input.set_value("");
            let user_str: &str = &user_title[..];

            if user_str.trim().len() == 0 || user_str.len() > MAX_PLAYLIST_TITLE_LENGTH {
              return;
            }
            counter += 1;
            set_playlist_items.update(|items| {
               items.push(PlaylistItem {
                name: user_title,
                id: counter
               });
            });
            set_playlist_dialog_state.update(|state| *state = false);
          }
        }>"Create playlist"</button>
      </div>
    </div>
  }
}
