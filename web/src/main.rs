#![feature(let_chains)]

use dioxus::prelude::*;

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
    let segments = use_resource(move || fetch::get_segments());

    let navigator = use_navigator();

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
                                th {
                                    "Retention in days"
                                }
                                th {
                                    "Created At"
                                }
                            }
                        }
                        tbody {
                            for segment in list {
                                tr {
                                    onclick: {
                                        let id = segment.segment.id.clone();
                                        move |_| {
                                            navigator.push(Route::Segment { id: id.to_string() });
                                        }
                                    },
                                    td {
                                        "{segment.segment.id}"
                                    }
                                    td {
                                        "{segment.segment.name}"
                                    }
                                    td {
                                        "{segment.segment.description}"
                                    }
                                    td {
                                       "{segment.segment.retention_in_days}"
                                    }
                                    td {
                                        "{segment.segment.created_at}"
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
        None => {
            rsx! {"Loading items"}
        }
    }
}

#[component]
fn Segment(id: String) -> Element {
    let segments = use_resource(move || fetch::get_segment(id.to_string()));

    match &*segments.read_unchecked() {
        Some(Ok(list)) => {
            if let Some(segment) = list.first() {
                rsx! {
                    div {
                        div {
                            "{segment.segment.id}"
                        }
                    }
                }
            } else {
                rsx! { "Segment Not Found" }
            }
        }
        Some(Err(err)) => {
            rsx! {"error: {err}"}
        }
        None => {
            rsx! { "Loading The Segment" }
        }
    }
}
