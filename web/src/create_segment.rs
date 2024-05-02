use std::str::FromStr;

use dioxus::prelude::*;

use snapchat::ty::segment::{DataSourceType, SegmentRequest, SegmentsRequest};

use crate::fetch;

#[component]
pub fn CreateSegments() -> Element {
    let segments: Vec<SegmentRequest> = vec![SegmentRequest {
        name: String::new(),
        description: String::new(),
        source_type: DataSourceType::FirstParty,
        retention_in_days: 0,
        ad_account_id: String::new(),
    }];
    let mut list = use_signal(|| segments);

    rsx! {
        div {
            class: "creates",
            h1 {
                "Create Segments"
            }
            div {
                class: "creates__list",
                for (index, segment) in list.read().iter().enumerate() {
                    div {
                        class: "create",
                        div {
                            class: "create__row",
                            div {
                                class: "create__row--label",
                                "Name"
                            }
                            input {
                                class: "create__row--input",
                                value: "{segment.name}",
                                oninput: move |e| {
                                    if let Some(new_seg) = list.write().get_mut(index) {
                                        new_seg.name = e.value();
                                    }
                                }
                            }
                        }
                        div {
                            class: "create__row",
                            div {
                                class: "create__row--label",
                                "Description"
                            }
                            input {
                                class: "create__row--input",
                                value: "{segment.description}",
                                oninput: move |e| {
                                    if let Some(new_seg) = list.write().get_mut(index) {
                                        new_seg.description = e.value();
                                    }
                                }
                            }
                        }
                        div {
                            class: "create__row",
                            div {
                                class: "create__row--label",
                                "Source Type"
                            }
                            select {
                                class: "create__row--input",
                                value: "{segment.source_type}",
                                oninput: move |e| {
                                    if let Some(new_seg) = list.write().get_mut(index) {
                                        let r = DataSourceType::from_str(&e.value());
                                        new_seg.source_type = if let Ok(_ty) = r {
                                            // FIXME(obei): Return `ty`.
                                            // Currently support `FirstParty` only
                                            DataSourceType::FirstParty
                                        } else {
                                            DataSourceType::FirstParty
                                        }
                                    }
                                },
                                option {
                                    value: "FIRST_PARTY",
                                    "First Party"
                                }
                                option {
                                    value: "ENGAGEMENT",
                                    "Engagement"
                                }
                                option {
                                    value: "PIXEL",
                                    "Pixel"
                                }
                                option {
                                    value: "MOBILE",
                                    "Mobile"
                                }
                                option {
                                    value: "FOOT_TRAFFIC_INSIGHTS",
                                    "FootTrafficInsights"
                                }
                            }
                        }
                        div {
                            class: "create__row",
                            div {
                                class: "create__row--label",
                                "Retention in days"
                            }
                            input {
                                class: "create__row--input",
                                value: "{segment.retention_in_days}",
                                oninput: move |e| {
                                    if let Some(new_seg) = list.write().get_mut(index) {
                                        let r = e.value().parse();
                                        new_seg.retention_in_days = if let Ok(number) = r {
                                            number
                                        } else {
                                            0
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "creates__control",
                div {
                    class: "creates__control-btn",
                    div {
                        class: "creates__control--add",
                        onclick: move |_| {
                            list.write().push(SegmentRequest {
                                name: String::new(),
                                description: String::new(),
                                source_type: DataSourceType::FirstParty,
                                retention_in_days: 0,
                                ad_account_id: String::new(),
                            });
                        },
                        button { "+" }
                    }
                    div {
                        class: "creates__control--minus",
                        onclick: move |_| {
                            list.write().pop();
                        },
                        button { "-" }
                    }
                }
                if !list.is_empty() {
                    button {
                        class: "creates__control--push",
                        onclick: move |_| {
                            let list: Vec<SegmentRequest> = list.read().clone();
                            let req = SegmentsRequest { segments: list };
                            let _ = use_resource(move || fetch::create_segments(req.clone()));
                        },
                        "Push to SnapChat"
                    }
                }
            }
        }
    }
}
