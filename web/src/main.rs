use dioxus::prelude::*;

use fetch::get_segments;

mod fetch;

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
    let segments = use_resource(move || get_segments());

    match &*segments.read_unchecked() {
        Some(Ok(list)) => {
            rsx! {
                div {
                    class: "segments",
                    h1 {
                        class: "segments__title",
                        "Segments List"
                    }
                    table {
                        class: "segments__table",
                        thead {
                            tr {
                                th {
                                    "ID"
                                }
                                th {
                                    "Name"
                                }
                                th {
                                    "Description"
                                }
                            }
                        }
                        tbody {
                            for segment in list {
                                tr {
                                    td {
                                        "{segment.segment.name}"
                                    }
                                    td {
                                        "name 1"
                                    }
                                    td {
                                        "desc 1"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx! { "error: {err}" }
        }
        _ => {
            rsx! {"Error loading items"}
        }
    }
}

#[component]
fn Segment(id: String) -> Element {
    rsx! {}
}
