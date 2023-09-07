use dioxus::prelude::*;

pub enum NotificationType {
    Loading, Info, Error
}

#[inline_props]
pub fn Notification<'a>(cx: Scope, notification_type: NotificationType,
                        title: &'a str, message: &'a str) -> Element {
    render! {
            div {
                class: "text-start",
                match notification_type {
                    NotificationType::Loading => {
                        rsx! {
                            div {
                                div {
                                    class: "fs-5",
                                    "{title}",
                                }
                            }
                        }
                    },
                    NotificationType::Info => {
                        rsx! {
                            div {
                                div {
                                    class: "fs-5 mb-2",
                                    "{title}",
                                },
                                "{message}"
                            }
                        }
                    },
                    NotificationType::Error => {
                        rsx! {
                            div {
                                div {
                                    class: "fs-5 mb-2 text-danger",
                                    "{title}",
                                },
                                "{message}"
                            }
                        }
                    }
                }
            }
    }
}