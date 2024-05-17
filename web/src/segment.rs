use dioxus::prelude::*;

use crate::fetch;

#[component]
pub fn Segment(id: String) -> Element {
    let segment_id = id.clone();
    let segments = use_resource(move || fetch::get_segment(segment_id.to_string()));

    let mut identifiers = use_signal(|| String::new());
    let mut schema_ty = use_signal(|| String::from("email"));

    match &*segments.read_unchecked() {
        Some(Ok(list)) => {
            if let Some(segment) = list.first() {
                rsx! {
                    div {
                        class: "segment-container",
                        h1 {
                            "Segment"
                        }
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
                            div {
                                select {
                                    class: "user__select",
                                    value: "{schema_ty}",
                                    oninput: move |e| schema_ty.set(e.value()),
                                    option {
                                        value: "email",
                                        "Email"
                                    }
                                    option {
                                        value: "mobile",
                                        "Mobile AD ID"
                                    }
                                    option {
                                        value: "phone",
                                        "Phone Number"
                                    }
                                }
                            }
                            input {
                                placeholder: "Add user identifiers for adding/removing from the segment. Separate them with commas",
                                value: "{identifiers}",
                                oninput: move |e| identifiers.set(e.value())
                            }
                            div {
                                class: "user__btn",
                                button {
                                    onclick: {
                                        let cloned_id = id.clone();
                                        move |_| {
                                            let cloned_id = cloned_id.clone();
                                            let vec: Vec<String> = identifiers.to_string().replace(' ', "").split(',').map(|s| s.to_string()).collect();
                                            let _ = use_resource(move || fetch::add_users(cloned_id.to_string(), vec.clone(), schema_ty.to_string()));
                                        }
                                    },
                                    "Add users"
                                }
                                button {
                                    onclick: {
                                        let cloned_id = id.clone();
                                        move |_| {
                                            let cloned_id = cloned_id.clone();
                                            let vec: Vec<String> = identifiers.to_string().replace(' ', "").split(',').map(|s| s.to_string()).collect();
                                            let _ = use_resource(move || fetch::delete_users(cloned_id.to_string(), vec.clone(), schema_ty.to_string()));
                                        }
                                    },
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
