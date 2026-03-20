use crate::state::{Progress, Screen, Theme};
use dioxus::prelude::*;

#[component]
pub fn SettingsScreen(
    mut current_screen: Signal<Screen>,
    mut progress: Signal<Progress>,
) -> Element {
    let current_theme = progress.read().theme;

    rsx! {
        div { class: "screen category-screen",
            // Header
            div { class: "screen-header",
                button {
                    class: "back-btn",
                    onclick: move |_| current_screen.set(Screen::Home),
                    "←"
                }
                h1 { class: "screen-title", "Settings" }
            }

            div { class: "category-content",
                h2 { class: "section-label", "Theme" }
                div { class: "theme-options",
                    for theme in &[Theme::System, Theme::Light, Theme::Dark] {
                        {
                            let theme = *theme;
                            let is_active = current_theme == theme;
                            let emoji = match theme {
                                Theme::System => "📱",
                                Theme::Light => "☀️",
                                Theme::Dark => "🌙",
                            };
                            rsx! {
                                button {
                                    class: if is_active { "theme-btn theme-btn--active" } else { "theme-btn" },
                                    onclick: move |_| {
                                        progress.write().theme = theme;
                                        crate::state::save_progress(&progress.read());
                                    },
                                    span { class: "theme-btn-emoji", "{emoji}" }
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
