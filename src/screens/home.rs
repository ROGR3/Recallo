use crate::state::{Language, Screen};
use crate::LOGO;
use dioxus::prelude::*;

#[component]
pub fn HomeScreen(mut current_screen: Signal<Screen>) -> Element {
    rsx! {
        div { class: "screen home-screen",
            div { class: "home-hero",
                img { class: "app-logo-img", src: LOGO, alt: "Recallo logo" }
                h1 { class: "app-title", "Recallo" }
                p { class: "app-subtitle", "Vocabulary Training" }
            }

            div { class: "language-section",
                h2 { class: "section-label", "Select Language" }
                div { class: "language-grid",
                    LanguageTile {
                        language: Language::Korean,
                        on_select: move |lang| {
                            current_screen.set(Screen::CategorySelect { language: lang });
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
fn LanguageTile(language: Language, on_select: EventHandler<Language>) -> Element {
    rsx! {
        button {
            class: "language-tile language-tile--active",
            onclick: move |_| on_select.call(language.clone()),
            span { class: "language-flag", "{language.flag()}" }
            span { class: "language-name", "{language.display_name()}" }
            span { class: "language-native", "{language.native_name()}" }
        }
    }
}
