use dioxus::prelude::*;

pub fn PageFooter(cx: Scope) -> Element {
    render! {
        div {
            class: "footer-links mt-5",
            span {
                class: "me-1",
                "v1.0.0"
            },
            span {
                class: "ms-1 me-1",
                "|"
            },
            a {
                class: "me-1 ms-1",
                href: "#",
                "HOW IT WORKS"
            }
            span {
                class: "ms-1 me-1",
                "|"
            },
            a {
                class: "ms-1",
                href: "#",
                "GITHUB"
            }
        }
    }
}