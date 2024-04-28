use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Auth {},
    #[route("/home")]
    Home {},
}

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        style { {include_str!("../assets/main.css")} }
        Router::<Route> {}
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
