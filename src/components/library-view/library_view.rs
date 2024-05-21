use std::fmt::Debug;
use leptos::{html::Input, *};
use web_sys::MouseEvent;
use crate::components::main_view::*;

mod playlist;

/// Represents the main component presenting the playlists and the songs
#[component]
pub fn LibraryView(
  set_sidebar_state: WriteSignal<bool>,
  set_playlist_items: WriteSignal<Vec<PlaylistItem>>,
) -> impl IntoView {
  let (playlist_dialog_state, set_playlist_dialog_state) = create_signal(false);
  let playlist_title_noderef = NodeRef::<Input>::new();

  /* Handles all the songs contained in playlists */
  let playlist_items: ReadSignal<Vec<PlaylistItem>> = {
    if let Some(items) = use_context::<ReadSignal<Vec<PlaylistItem>>>() {
      items
    } else {
      context_val_not_provided("playlist_items")
    }
  };

  /* Handles the state of the sidebar */
  let sidebar_state: ReadSignal<bool> = {
    if let Some(state) = use_context::<ReadSignal<bool>>() {
      state
    } else {
      context_val_not_provided("sidebar_state")
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
          <button id="sidebar-toggle-button" on:click= move |_:MouseEvent| {
            set_sidebar_state.update(|current_state| *current_state = !(*current_state));
          }>
          <img alt="toggle navigation state" src= move || {
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
          <button id="playlist-create-button" on:click= move|_| {
            set_playlist_dialog_state.update(|state| *state = true)
          }>
            "Create playlist"
          </button>
          <hr/>
        </hgroup>
      </div>
      <div id="playlist-container">
        <Show when= move|| playlist_items.get().is_empty()>
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

    <div id="playlist-create-dialog" data-visible= move|| {
      if playlist_dialog_state.get(){
        "visible"
      } else {
        "hidden"
      }
    }>
      <div id="dialog-closer" on:click= move|_| {
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

fn context_val_not_provided(value_name: impl Debug) -> ! {
  panic!("The \"{:?}\" context value was not porovided", value_name);
}