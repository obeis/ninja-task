#![feature(let_chains)]

use dioxus::prelude::*;

use crate::create_segment::CreateSegments;
use crate::segment::Segment;
use crate::segments::Segments;
use crate::update_segment::UpdateSegment;

mod create_segment;
mod fetch;
mod segment;
mod segments;
mod update_segment;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Segments {},
    #[route("/segment/:id")]
    Segment { id: String },
    #[route("/create")]
    CreateSegments,
    #[route("/update/:id")]
    UpdateSegment { id: String },
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
