use crate::state::{GameConfig, GameMode, Progress, Screen, PENALTY_SECONDS};
use crate::util::{format_time, go_back, navigate};
use dioxus::prelude::*;

#[component]
pub fn ModeSelectScreen(
    config: GameConfig,
    mut current_screen: Signal<Screen>,
    mut history: Signal<Vec<Screen>>,
    progress: Signal<Progress>,
) -> Element {
    let cat_display = config.category_display_name();
    let penalty = PENALTY_SECONDS as u32;

    rsx! {
        div { class: "screen category-screen",
            div { class: "screen-header",
                button {
                    class: "back-btn",
                    onclick: move |_| go_back(&mut current_screen, &mut history),
                    "\u{2190}"
                }
                h1 { class: "screen-title",
                    "{config.subject.flag()} {cat_display}"
                }
            }

            div { class: "category-content",
                h2 { class: "section-label", "Zvol re\u{017e}im" }

                div { class: "mode-grid",
                    for mode in [GameMode::Words10, GameMode::Words20] {
                        {
                            let mut cfg = config.clone();
                            cfg.mode = mode;
                            let best_key = cfg.best_time_key();
                            let best = progress.read().get_best_time(&best_key);
                            rsx! {
                                ModeCard {
                                    mode,
                                    best_time: best,
                                    on_select: {
                                        let cfg = cfg.clone();
                                        move |_| {
                                            navigate(&mut current_screen, &mut history, Screen::Game { config: cfg.clone() });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "mode-hint",
                    "+{penalty}s penalizace za chybu"
                }
            }
        }
    }
}

#[component]
fn ModeCard(mode: GameMode, best_time: Option<f64>, on_select: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "mode-card",
            onclick: move |_| on_select.call(()),
            div { class: "mode-card-header",
                span { class: "mode-card-emoji", "{mode.emoji()}" }
                span { class: "mode-card-title", "{mode.display_name()}" }
            }
            p { class: "mode-card-subtitle", "{mode.subtitle()}" }
            if let Some(t) = best_time {
                div { class: "mode-card-best",
                    span { class: "mode-card-best-label", "Rekord" }
                    span { class: "mode-card-best-time", "{format_time(t)}" }
                }
            } else {
                div { class: "mode-card-best mode-card-best--empty",
                    span { "\u{017d}\u{00e1}dn\u{00fd} rekord" }
                }
            }
        }
    }
}
