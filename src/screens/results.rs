use crate::state::{GameResult, Screen};
use crate::util::format_time;
use dioxus::prelude::*;

#[component]
pub fn ResultsScreen(result: GameResult, mut current_screen: Signal<Screen>) -> Element {
    let pct = if result.total > 0 {
        (result.score as f64 / result.total as f64 * 100.0) as u32
    } else {
        0
    };

    let (grade_emoji, grade_title, grade_class) = match pct {
        90..=100 => ("\u{1f3c6}", "V\u{00fd}born\u{011b}!", "grade--gold"),
        70..=89 => ("\u{2b50}", "Skv\u{011b}l\u{00e1} pr\u{00e1}ce!", "grade--silver"),
        50..=69 => ("\u{1f44d}", "Dobr\u{00e1} snaha!", "grade--bronze"),
        _ => ("\u{1f4aa}", "Cvi\u{010d} d\u{00e1}l!", "grade--default"),
    };

    let time_str = format_time(result.time_seconds);
    let cat_display = result.config.category_display_name();
    let mode_name = result.config.mode.display_name();
    let has_penalty = result.penalty_seconds > 0.0;
    let penalty_str = format_time(result.penalty_seconds);
    let mistakes_label = if result.mistakes == 1 {
        "1 chyba".to_string()
    } else if result.mistakes >= 2 && result.mistakes <= 4 {
        format!("{} chyby", result.mistakes)
    } else {
        format!("{} chyb", result.mistakes)
    };

    rsx! {
        div { class: "screen results-screen",
            div { class: "results-hero",
                span { class: "results-grade-emoji", "{grade_emoji}" }
                h1 { class: "results-title {grade_class}", "{grade_title}" }
                p { class: "results-subtitle", "{cat_display} \u{00b7} {mode_name}" }
            }

            div { class: "results-stats",
                div { class: "stat-card",
                    span { class: "stat-label", "Sk\u{00f3}re" }
                    span { class: "stat-value stat-value--score",
                        "{result.score}"
                        span { class: "stat-total", " / {result.total}" }
                    }
                    span { class: "stat-pct", "{pct}%" }
                }

                div { class: "stat-card",
                    span { class: "stat-label", "\u{010c}as" }
                    span { class: "stat-value", "{time_str}" }
                    if result.is_new_best {
                        span { class: "best-badge", "Nov\u{00fd} rekord!" }
                    } else if let Some(pb) = result.previous_best {
                        span { class: "stat-hint",
                            "Rekord: {format_time(pb)}"
                        }
                    }
                }

                if has_penalty {
                    div { class: "stat-card stat-card--penalty",
                        span { class: "stat-label", "Penalizace" }
                        span { class: "stat-value stat-value--penalty", "+{penalty_str}" }
                        span { class: "stat-hint stat-hint--penalty", "{mistakes_label}" }
                    }
                }
            }

            div { class: "results-actions",
                button {
                    class: "start-btn",
                    onclick: {
                        let config = result.config.clone();
                        move |_| {
                            current_screen.set(Screen::Game { config: config.clone() });
                        }
                    },
                    "Hr\u{00e1}t znovu"
                }
                button {
                    class: "secondary-btn",
                    onclick: {
                        let config = result.config.clone();
                        move |_| {
                            current_screen.set(Screen::ModeSelect { config: config.clone() });
                        }
                    },
                    "Zm\u{011b}nit re\u{017e}im"
                }
                button {
                    class: "ghost-btn",
                    onclick: move |_| current_screen.set(Screen::Home),
                    "Hlavn\u{00ed}"
                }
            }
        }
    }
}
