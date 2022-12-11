use dioxus::prelude::*;

pub fn header(cx: Scope) -> Element {
  cx.render(rsx!(
      h1 {
        "Button Components"
      }
  ))
}