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
    let segment_id = id.clone();
    let segments = use_resource(move || fetch::get_segment(segment_id.to_string()));

    match &*segments.read_unchecked() {
        Some(Ok(list)) => {
            if let Some(segment) = list.first() {
                rsx! {
                    div {
                        class: "segment-container",
                        div {
                            class: "segment",
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "ID"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.id}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Name"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.name}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Description"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.description}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Status"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.status}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Source Type"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.source_type}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Ad Account ID"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.ad_account_id}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Organization ID"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.organization_id}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Targetable Status"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.targetable_status}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Upload Status"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.upload_status}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Retention In Days"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.retention_in_days}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Approximate Number Of Users"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.approximate_number_users}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Visible To"
                                }
                                div {
                                    class: "segment__info--value",
                                    r#"{segment.segment.visible_to.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ")}"#
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Created At"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.created_at}"
                                }
                            }
                            div {
                                class: "segment__info",
                                div {
                                    class: "segment__info--label",
                                    "Updated At"
                                }
                                div {
                                    class: "segment__info--value",
                                    "{segment.segment.updated_at}"
                                }
                            }
                        }

                        div {
                            class: "user",
                            input {
                                placeholder: "add emails to add/remove. e.g: email1, email2",
                            }
                            div {
                                class: "user__btn",
                                button {
                                    "Add users"
                                }
                                button {
                                    "Remove users"
                                }
                            }
                        }

                        button {
                            onclick: {
                                let cloned_id = id.clone();
                                move |_| {
                                    let cloned_id = cloned_id.clone();
                                    let _ = use_resource(move || fetch::delete_all_users(cloned_id.to_string()));
                                }
                            },
                            "Delete All users from the segment"
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
