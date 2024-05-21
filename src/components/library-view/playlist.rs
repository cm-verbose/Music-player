use leptos::*;
use rand::{seq::SliceRandom, Rng};

/// Represents a playlist seen on the library page
#[component]
pub fn Playlist(name: String) -> impl IntoView {
  view! {
    <div class="playlist">
      <SVGElement name={name.to_owned()}/>
      <section>
        <h2>{name}</h2>
      </section>
    </div>
  }
}

#[component]
fn SVGElement(name: String) -> impl IntoView {
  const MAX_NAME_LENGTH_SVG: usize = 6; 
  const COLOR_LIST: [u32; 4] = [0xABE0AD, 0x81D8D0, 0x5A4FCF, 0xFF69B4];
  let color_int: u32 = *COLOR_LIST.choose(&mut rand::thread_rng()).unwrap();
  let [h, s, l]: [f32; 3] = hsl_rotate_from_hex(color_int, rand::thread_rng().gen());
  let hsl_str = format!("hsl({h}, {s}%, {l}%)");

  let SVGStyles = leptos::svg::style();
  SVGStyles.set_inner_html("
    text {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
      max-width: 250px; 
    }
    "
  );

  view!{
    <svg width={100}>
      {SVGStyles}
      <rect width="100%" height="100%" fill={hsl_str}/>
      <text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle"
        fill={format!("hsl({h}, {s}%, {}%)", l * 0.5)}>
        {{
          let mut result: Vec<char> = vec![]; 
          let mut i = 0; 
          for chr in name.chars(){
            if chr.is_whitespace() {
              continue 
            }; 
            result.push(chr); 
            if i == MAX_NAME_LENGTH_SVG - 1 {
              break 
            }; 
            i += 1; 
          }
          result.iter().collect::<String>()
        }}
      </text>
    </svg>
  }
}

/// Rotates a hex color represented as a [u32] and returns a hsl triple 
fn hsl_rotate_from_hex(hex: u32, rotation: f32) -> [f32; 3] {
  let r = (((hex >> 16) & 0xFF) as f32) / 255.;
  let g = (((hex >> 8) & 0xFF) as f32) / 255.;
  let b = ((hex & 0xFF) as f32) / 255.;

  let max = r.max(g.max(b));
  let min = r.min(g.min(b));
  let diff = max - min;
  let l = ((max + min) / 2.) * 100.;

  let s = (diff / if l > 0.5 { 2.0 - max - min } else { max + min }) * 100.0;
  let h = if max == min {
    0.
  } else {
    let c = max - min;
    (if max == r {
      (g - b) / c + (if g < b { 6.0 } else { 0.0 })
    } else if max == g {
      (b - r) / c + 2.0
    } else {
      (r - g) / c + 4.0
    }) / 6.0
  } * 360.;

  [(h + (rotation * 360.)) % 360., s, l].map(|n| n.round())
}
