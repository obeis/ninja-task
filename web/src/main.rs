use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/auth")]
    Auth {},
    #[route("/auth/:code")]
    AuthCode { code: String },
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
fn Home() -> Element {
    rsx! {
        h1 { "Home" }
    }
}

#[component]
fn Auth() -> Element {
    rsx! {
        div {
            class: "auth",
            h1 {
                class: "auth__title",
                "Connect your SnapChat"
            }

            a {
                href: "https://accounts.snapchat.com/login/oauth2/authorize?client_id=fce56ca5-49a1-4395-bf48-574744af4905&redirect_uri=https://api.khwarizmi.io/v1/auth&response_type=code&scope=snapchat-marketing-api snapchat-offline-conversions-api snapchat-profile-api",
                button {
                    class: "auth__button",
                    "connect"
                }
            }
        }
    }
}

#[component]
fn AuthCode(code: String) -> Element {
    rsx! {
        h1 { "Auth Code: {code}" }
    }
}
