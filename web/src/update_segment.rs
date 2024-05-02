use dioxus::prelude::*;
use snapchat::ty::segment::UpdateSegmentRequest;

use crate::fetch;

#[component]
pub fn UpdateSegment(id: String) -> Element {
    let segment_id = id.clone();
    let segments = use_resource(move || fetch::get_segment(segment_id.to_string()));

    let mut segment_signal = use_signal(|| UpdateSegmentRequest {
        id,
        name: String::new(),
        description: String::new(),
        retention_in_days: 0,
        ad_account_id: String::new(),
    });

    match &*segments.read_unchecked() {
        Some(Ok(list)) => {
            if let Some(segment) = list.first() {
                segment_signal
                    .write()
                    .name
                    .clone_from(&segment.segment.name);
                segment_signal
                    .write()
                    .description
                    .clone_from(&segment.segment.description);
                segment_signal
                    .write()
                    .retention_in_days
                    .clone_from(&segment.segment.retention_in_days);
                rsx! {
                    div {
                        class: "update",
                        h1 { "Update Segment" }
                        div {
                            class: "update__row",
                            div {
                                class: "update__row--label",
                                "Name"
                            }
                            input {
                                class: "update__row--input",
                                value: "{segment.segment.name}",
                                oninput: move |e| {
                                    segment_signal.write().name = e.value();
                                }
                            }
                        }
                        div {
                            class: "update__row",
                            div {
                                class: "update__row--label",
                                "Description"
                            }
                            input {
                                class: "update__row--input",
                                value: "{segment.segment.description}",
                                oninput: move |e| {
                                    segment_signal.write().description = e.value();
                                }
                            }
                        }
                        div {
                            class: "update__row",
                            div {
                                class: "update__row--label",
                                "Retention in days"
                            }
                            input {
                                class: "update__row--input",
                                value: "{segment.segment.retention_in_days}",
                                oninput: move |e| {
                                    let r = e.value().parse();
                                    segment_signal.write().retention_in_days = if let Ok(number) = r {
                                        number
                                    } else {
                                        0
                                    };
                                }
                            }
                        }
                        div {
                            class: "update__btn",
                            button {
                                onclick: {
                                    move |_| {
                                        let req: UpdateSegmentRequest = segment_signal.read().clone();
                                        let _ = use_resource(move || fetch::update_segment(req.clone()));
                                    }
                                },
                                "Update"
                            }
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
