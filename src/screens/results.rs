use crate::state::{GameResult, Screen};
use dioxus::prelude::*;

fn format_time(secs: f64) -> String {
    let mins = (secs / 60.0) as u32;
    let sec_part = (secs % 60.0) as u32;
    let ms = ((secs % 1.0) * 10.0) as u32;
    if mins > 0 {
        format!("{:02}:{:02}.{}", mins, sec_part, ms)
    } else {
        format!("{}.{}s", sec_part, ms)
    }
}

#[component]
pub fn ResultsScreen(result: GameResult, mut current_screen: Signal<Screen>) -> Element {
    let pct = if result.total > 0 {
        (result.score as f64 / result.total as f64 * 100.0) as u32
    } else {
        0
    };

    let grade = if result.failed_perfect {
        ("💥", "Failed!", "grade--failed")
    } else {
        match pct {
            90..=100 => ("🏆", "Excellent!", "grade--gold"),
            70..=89 => ("⭐", "Great job!", "grade--silver"),
            50..=69 => ("👍", "Good effort!", "grade--bronze"),
            _ => ("💪", "Keep practicing!", "grade--default"),
        }
    };

    let time_str = format_time(result.time_seconds);
    let cat_display = result
        .config
        .category
        .as_ref()
        .map(|c| c.display_name())
        .unwrap_or("All");
    let mode_name = result.config.mode.display_name();

    rsx! {
        div { class: "screen results-screen",
            div { class: "results-hero",
                span { class: "results-grade-emoji", "{grade.0}" }
                h1 { class: "results-title {grade.2}", "{grade.1}" }
                p { class: "results-subtitle", "{cat_display} · {mode_name}" }
            }

            div { class: "results-stats",
                div { class: "stat-card",
                    span { class: "stat-label", "Score" }
                    span { class: "stat-value stat-value--score",
                        "{result.score}"
                        span { class: "stat-total", " / {result.total}" }
                    }
                    if !result.failed_perfect {
                        span { class: "stat-pct", "{pct}%" }
                    }
                }

                div { class: "stat-card",
                    span { class: "stat-label", "Time" }
                    span { class: "stat-value", "{time_str}" }
                    if result.failed_perfect {
                        span { class: "stat-hint stat-hint--fail", "Not recorded (failed)" }
                    } else if result.is_new_best {
                        span { class: "best-badge", "🎉 New Best!" }
                    } else if let Some(pb) = result.previous_best {
                        span { class: "stat-hint",
                            "Best: {format_time(pb)}"
                        }
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
                    if result.failed_perfect { "Try Again" } else { "Play Again" }
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
