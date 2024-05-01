#![feature(let_chains)]

use dioxus::prelude::*;

use crate::create_segment::CreateSegments;
use crate::segment::Segment;
use crate::segments::Segments;

mod create_segment;
mod fetch;
mod segment;
mod segments;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Segments {},
    #[route("/segment/:id")]
    Segment { id: String },
    #[route("/create")]
    CreateSegments,
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
