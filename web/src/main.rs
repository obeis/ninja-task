use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        style { {include_str!("../assets/main.css")} }

        Auth {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        h1 { "Home" }
    }
}

#[component]
fn Auth() -> Element {
    rsx! {
        h1 { "Auth" }
    }
}
