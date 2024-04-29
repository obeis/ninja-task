use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Segments {},
    #[route("/segment/:id")]
    Segment { id: String },
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
fn Segments() -> Element {
    rsx! {}
}

#[component]
fn Segment(id: String) -> Element {
    rsx! {}
}
