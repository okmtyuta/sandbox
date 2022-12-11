use dioxus::prelude::*;

pub fn button(cx: Scope) -> Element {
  
  cx.render(rsx!(
      button {
        "Button Components"
      }
  ))
}