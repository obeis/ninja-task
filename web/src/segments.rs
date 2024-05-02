use std::collections::HashSet;

use dioxus::prelude::*;

use crate::fetch;
use crate::Route;

#[component]
pub fn Segments() -> Element {
    let segments = use_resource(move || fetch::get_segments());

    let navigator = use_navigator();

    let m: HashSet<String> = HashSet::new();
    let mut signal = use_signal(move || m);

    match &*segments.read_unchecked() {
        Some(Ok(list)) => {
            let cloned_map = signal.read().clone();
            let len = list.len() - cloned_map.len();
            rsx! {
                div {
                    class: "segments",
                    h1 {
                        class: "segments__title",
                        "Segments List ({len})"
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
                                th {
                                    "Actions"
                                }
                            }
                        }
                        tbody {
                            for segment in list.iter().filter(|seg| !cloned_map.contains(&seg.segment.id) ) {
                                tr {
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
                                    td {
                                        span {
                                            onclick: {
                                                let id = segment.segment.id.clone();
                                                move |_| {
                                                    navigator.push(Route::Segment { id: id.to_string() });
                                                }
                                            },
                                            img {
                                                src: "data:image/svg+xml;base64,PHN2ZyBjbGlwLXJ1bGU9ImV2ZW5vZGQiIGZpbGwtcnVsZT0iZXZlbm9kZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgc3Ryb2tlLW1pdGVybGltaXQ9IjIiIHZpZXdCb3g9IjAgMCAyNCAyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cGF0aCBkPSJtMTEuOTk4IDVjLTQuMDc4IDAtNy43NDIgMy4wOTMtOS44NTMgNi40ODMtLjA5Ni4xNTktLjE0NS4zMzgtLjE0NS41MTdzLjA0OC4zNTguMTQ0LjUxN2MyLjExMiAzLjM5IDUuNzc2IDYuNDgzIDkuODU0IDYuNDgzIDQuMTQzIDAgNy43OTYtMy4wOSA5Ljg2NC02LjQ5My4wOTItLjE1Ni4xMzgtLjMzMi4xMzgtLjUwN3MtLjA0Ni0uMzUxLS4xMzgtLjUwN2MtMi4wNjgtMy40MDMtNS43MjEtNi40OTMtOS44NjQtNi40OTN6bS4wMDIgM2MyLjIwOCAwIDQgMS43OTIgNCA0cy0xLjc5MiA0LTQgNC00LTEuNzkyLTQtNCAxLjc5Mi00IDQtNHptMCAxLjVjMS4zOCAwIDIuNSAxLjEyIDIuNSAyLjVzLTEuMTIgMi41LTIuNSAyLjUtMi41LTEuMTItMi41LTIuNSAxLjEyLTIuNSAyLjUtMi41eiIgZmlsbC1ydWxlPSJub256ZXJvIi8+PC9zdmc+"
                                            }
                                        }
                                        span {
                                            onclick: {
                                                let id = segment.segment.id.clone();
                                                move |_| {
                                                    navigator.push(Route::UpdateSegment { id: id.to_string() });
                                                }
                                            },
                                            img {
                                                src: "data:image/svg+xml;base64,PHN2ZyBjbGlwLXJ1bGU9ImV2ZW5vZGQiIGZpbGwtcnVsZT0iZXZlbm9kZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgc3Ryb2tlLW1pdGVybGltaXQ9IjIiIHZpZXdCb3g9IjAgMCAyNCAyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48cGF0aCBkPSJtNC40ODEgMTUuNjU5Yy0xLjMzNCAzLjkxNi0xLjQ4IDQuMjMyLTEuNDggNC41ODcgMCAuNTI4LjQ2Ljc0OS43NDkuNzQ5LjM1MiAwIC42NjgtLjEzNyA0LjU3NC0xLjQ5MnptMS4wNi0xLjA2MSAzLjg0NiAzLjg0NiAxMS4zMjEtMTEuMzExYy4xOTUtLjE5NS4yOTMtLjQ1LjI5My0uNzA3IDAtLjI1NS0uMDk4LS41MS0uMjkzLS43MDYtLjY5Mi0uNjkxLTEuNzQyLTEuNzQtMi40MzUtMi40MzItLjE5NS0uMTk1LS40NTEtLjI5My0uNzA3LS4yOTMtLjI1NCAwLS41MS4wOTgtLjcwNi4yOTN6IiBmaWxsLXJ1bGU9Im5vbnplcm8iLz48L3N2Zz4="
                                            }
                                        }
                                        span {
                                            onclick: {
                                                let id = segment.segment.id.clone();
                                                move |_| {
                                                    let cloned_id = id.clone();
                                                    let map_id = id.clone();
                                                    let _ = use_resource(move || fetch::delete_segment(cloned_id.clone()));
                                                    signal.write().insert(map_id);
                                                }
                                            },
                                            img {
                                                src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0Ij48cGF0aCBkPSJNMyA2djE4aDE4di0xOGgtMTh6bTUgMTRjMCAuNTUyLS40NDggMS0xIDFzLTEtLjQ0OC0xLTF2LTEwYzAtLjU1Mi40NDgtMSAxLTFzMSAuNDQ4IDEgMXYxMHptNSAwYzAgLjU1Mi0uNDQ4IDEtMSAxcy0xLS40NDgtMS0xdi0xMGMwLS41NTIuNDQ4LTEgMS0xczEgLjQ0OCAxIDF2MTB6bTUgMGMwIC41NTItLjQ0OCAxLTEgMXMtMS0uNDQ4LTEtMXYtMTBjMC0uNTUyLjQ0OC0xIDEtMXMxIC40NDggMSAxdjEwem00LTE4djJoLTIwdi0yaDUuNzExYy45IDAgMS42MzEtMS4wOTkgMS42MzEtMmg1LjMxNWMwIC45MDEuNzMgMiAxLjYzMSAyaDUuNzEyeiIvPjwvc3ZnPg=="
                                            }
                                        }
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
