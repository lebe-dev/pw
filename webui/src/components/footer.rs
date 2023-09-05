use dioxus::prelude::*;

#[inline_props]
pub fn PageFooter<'a>(cx: Scope, how_it_works_label: &'a str, locale_id: &'a str) -> Element {
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
                class: "text-uppercase me-1 ms-1",
                href: "https://github.com/lebe-dev/pw/blob/main/docs/faq/FAQ.{cx.props.locale_id}.md",
                target: "_blank",
                "{cx.props.how_it_works_label}"
            }
            span {
                class: "ms-1 me-1",
                "|"
            },
            a {
                class: "ms-1",
                href: "https://github.com/lebe-dev/pw",
                target: "_blank",
                "GITHUB"
            }
        }
    }
}