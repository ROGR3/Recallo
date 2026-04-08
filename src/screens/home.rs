use crate::state::{Screen, Subject};
use crate::util::navigate;
use crate::LOGO;
use dioxus::prelude::*;

#[component]
pub fn HomeScreen(mut current_screen: Signal<Screen>, mut history: Signal<Vec<Screen>>) -> Element {
    rsx! {
        div { class: "screen home-screen",
            div { class: "home-top-bar",
                div {}
                button {
                    class: "settings-btn",
                    onclick: move |_| navigate(&mut current_screen, &mut history, Screen::Settings),
                    "\u{2699}"
                }
            }

            div { class: "home-hero",
                img { class: "app-logo-img", src: LOGO, alt: "Recallo logo" }
                h1 { class: "app-title", "Recallo" }
                p { class: "app-subtitle", "Tr\u{00e9}nink slovn\u{00ed} z\u{00e1}soby" }
            }

            div { class: "language-section",
                h2 { class: "section-label", "Zvol p\u{0159}edm\u{011b}t" }
                div { class: "language-grid",
                    for subject in [Subject::Korean, Subject::MathAnalysis, Subject::MathDataScience] {
                        SubjectTile {
                            subject,
                            on_select: move |subj: Subject| {
                                navigate(&mut current_screen, &mut history, Screen::CategorySelect { subject: subj });
                            }
                        }
                    }
                }
            }

            div { class: "home-footer",
                p { "Na \u{010d}as \u{00b7} sleduj sv\u{00e9} rekordy" }
            }
        }
    }
}

#[component]
fn SubjectTile(subject: Subject, on_select: EventHandler<Subject>) -> Element {
    rsx! {
        button {
            class: "language-tile language-tile--active",
            onclick: move |_| on_select.call(subject),
            span { class: "language-flag", "{subject.flag()}" }
            span { class: "language-name", "{subject.display_name()}" }
            span { class: "language-native", "{subject.native_name()}" }
        }
    }
}
