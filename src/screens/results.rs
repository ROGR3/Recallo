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
        90..=100 => ("\u{1f3c6}", "Excellent!", "grade--gold"),
        70..=89 => ("\u{2b50}", "Great job!", "grade--silver"),
        50..=69 => ("\u{1f44d}", "Good effort!", "grade--bronze"),
        _ => ("\u{1f4aa}", "Keep practicing!", "grade--default"),
    };

    let time_str = format_time(result.time_seconds);
    let cat_display = result.config.category_display_name();
    let mode_name = result.config.mode.display_name();
    let has_penalty = result.penalty_seconds > 0.0;
    let penalty_str = format_time(result.penalty_seconds);
    let mistakes_label = if result.mistakes == 1 {
        "1 mistake".to_string()
    } else {
        format!("{} mistakes", result.mistakes)
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
                    span { class: "stat-label", "Score" }
                    span { class: "stat-value stat-value--score",
                        "{result.score}"
                        span { class: "stat-total", " / {result.total}" }
                    }
                    span { class: "stat-pct", "{pct}%" }
                }

                div { class: "stat-card",
                    span { class: "stat-label", "Time" }
                    span { class: "stat-value", "{time_str}" }
                    if result.is_new_best {
                        span { class: "best-badge", "New Best!" }
                    } else if let Some(pb) = result.previous_best {
                        span { class: "stat-hint",
                            "Best: {format_time(pb)}"
                        }
                    }
                }

                if has_penalty {
                    div { class: "stat-card stat-card--penalty",
                        span { class: "stat-label", "Penalty" }
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
                    "Play Again"
                }
                button {
                    class: "secondary-btn",
                    onclick: {
                        let config = result.config.clone();
                        move |_| {
                            current_screen.set(Screen::ModeSelect { config: config.clone() });
                        }
                    },
                    "Change Mode"
                }
                button {
                    class: "ghost-btn",
                    onclick: move |_| current_screen.set(Screen::Home),
                    "Home"
                }
            }
        }
    }
}
