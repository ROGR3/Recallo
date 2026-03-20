use crate::state::{Screen, Subject};
use crate::LOGO;
use dioxus::prelude::*;

#[component]
pub fn HomeScreen(mut current_screen: Signal<Screen>) -> Element {
    rsx! {
        div { class: "screen home-screen",
            // Settings gear in top-right
            div { class: "home-top-bar",
                div {}
                button {
                    class: "settings-btn",
                    onclick: move |_| current_screen.set(Screen::Settings),
                    "⚙"
                }
            }

            div { class: "home-hero",
                img { class: "app-logo-img", src: LOGO, alt: "Recallo logo" }
                h1 { class: "app-title", "Recallo" }
                p { class: "app-subtitle", "Vocabulary Training" }
            }

            div { class: "language-section",
                h2 { class: "section-label", "Select Subject" }
                div { class: "language-grid",
                    SubjectTile {
                        subject: Subject::Korean,
                        on_select: move |subj| {
                            current_screen.set(Screen::CategorySelect { subject: subj });
                        }
                    }
                    SubjectTile {
                        subject: Subject::MathAnalysis,
                        on_select: move |subj| {
                            current_screen.set(Screen::CategorySelect { subject: subj });
                        }
                    }
                    SubjectTile {
                        subject: Subject::MathDataScience,
                        on_select: move |subj| {
                            current_screen.set(Screen::CategorySelect { subject: subj });
                        }
                    }
                }
            }

            div { class: "home-footer",
                p { "Timed · track your best" }
            }
        }
    }
}

#[component]
fn SubjectTile(subject: Subject, on_select: EventHandler<Subject>) -> Element {
    rsx! {
        button {
            class: "language-tile language-tile--active",
            onclick: move |_| on_select.call(subject.clone()),
            span { class: "language-flag", "{subject.flag()}" }
            span { class: "language-name", "{subject.display_name()}" }
            span { class: "language-native", "{subject.native_name()}" }
        }
    }
}
