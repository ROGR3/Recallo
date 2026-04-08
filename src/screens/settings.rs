use crate::state::{Progress, Screen, Theme};
use crate::util::go_back;
use dioxus::prelude::*;

#[component]
pub fn SettingsScreen(
    mut current_screen: Signal<Screen>,
    mut history: Signal<Vec<Screen>>,
    mut progress: Signal<Progress>,
) -> Element {
    let current_theme = progress.read().theme;

    rsx! {
        div { class: "screen category-screen",
            div { class: "screen-header",
                button {
                    class: "back-btn",
                    onclick: move |_| go_back(&mut current_screen, &mut history),
                    "\u{2190}"
                }
                h1 { class: "screen-title", "Settings" }
            }

            div { class: "category-content",
                h2 { class: "section-label", "Theme" }
                div { class: "theme-options",
                    for theme in [Theme::System, Theme::Light, Theme::Dark] {
                        {
                            let is_active = current_theme == theme;
                            rsx! {
                                button {
                                    class: if is_active { "theme-btn theme-btn--active" } else { "theme-btn" },
                                    onclick: move |_| {
                                        progress.write().theme = theme;
                                        crate::state::save_progress(&progress.read());
                                    },
                                    span { class: "theme-btn-emoji", "{theme.emoji()}" }
                                    span { class: "theme-btn-label", "{theme.display_name()}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
