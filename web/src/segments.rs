use dioxus::prelude::*;

use crate::fetch;
use crate::Route;

#[component]
pub fn Segments() -> Element {
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
                    div {
                        class: "segments__control",
                        div {
                            class: "segments__control--add",
                            onclick: move |_| {
                                navigator.push(Route::CreateSegments {});
                            },
                            "+"
                        }
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
