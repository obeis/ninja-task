#![feature(let_chains)]

use dioxus::prelude::*;

use crate::segment::Segment;
use crate::segments::Segments;

mod fetch;
mod segment;
mod segments;

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
