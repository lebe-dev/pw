use dioxus::prelude::*;

pub fn PageHeader(cx: Scope) -> Element {
    render! {
        nav {
                class: "navbar bg-dark text-info",
                div {
                    class: "container-fluid",
                    a {
                        class: "navbar-brand text-light cursor-pointer",
                        title: "Back to home page",
                        href: "/",
                        "PW"
                    }
                }
        }
    }
}