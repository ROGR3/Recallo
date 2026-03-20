use crate::data::{Category, MathTopic, MATH_ENTRIES, WORDS};
use crate::state::{GameConfig, GameMode, Progress, Screen, Subject};
use dioxus::prelude::*;

/// A unified category info item used for both Korean categories and Math topics.
#[derive(Clone, PartialEq)]
struct CategoryInfo {
    key: String,
    display_name: String,
    emoji: String,
}

/// Get category info list for a given subject.
fn categories_for_subject(subject: &Subject) -> Vec<CategoryInfo> {
    match subject {
        Subject::Korean => Category::all()
            .iter()
            .map(|c| CategoryInfo {
                key: c.display_name().to_lowercase(),
                display_name: c.display_name().to_string(),
                emoji: c.emoji().to_string(),
            })
            .collect(),
        Subject::MathAnalysis => MathTopic::all_for_subject(crate::data::MathSubject::Analysis)
            .into_iter()
            .map(|t| CategoryInfo {
                key: t.key().to_string(),
                display_name: t.display_name().to_string(),
                emoji: t.emoji().to_string(),
            })
            .collect(),
        Subject::MathDataScience => {
            MathTopic::all_for_subject(crate::data::MathSubject::DataScience)
                .into_iter()
                .map(|t| CategoryInfo {
                    key: t.key().to_string(),
                    display_name: t.display_name().to_string(),
                    emoji: t.emoji().to_string(),
                })
                .collect()
        }
    }
}

/// Count total entries for a subject, optionally filtered by category key.
fn count_entries(subject: &Subject, category_key: Option<&str>) -> usize {
    match subject {
        Subject::Korean => WORDS
            .iter()
            .filter(|w| {
                category_key
                    .as_ref()
                    .is_none_or(|k| w.category.display_name().to_lowercase() == *k)
            })
            .count(),
        Subject::MathAnalysis => MATH_ENTRIES
            .iter()
            .filter(|e| e.subject == crate::data::MathSubject::Analysis)
            .filter(|e| category_key.as_ref().is_none_or(|k| e.topic.key() == *k))
            .count(),
        Subject::MathDataScience => MATH_ENTRIES
            .iter()
            .filter(|e| e.subject == crate::data::MathSubject::DataScience)
            .filter(|e| category_key.as_ref().is_none_or(|k| e.topic.key() == *k))
            .count(),
    }
}

/// Count known entries for a subject, optionally filtered by category key.
fn count_known(subject: &Subject, category_key: Option<&str>, progress: &Progress) -> usize {
    match subject {
        Subject::Korean => WORDS
            .iter()
            .filter(|w| {
                category_key
                    .as_ref()
                    .is_none_or(|k| w.category.display_name().to_lowercase() == *k)
            })
            .filter(|w| progress.is_known(w.korean))
            .count(),
        Subject::MathAnalysis => MATH_ENTRIES
            .iter()
            .filter(|e| e.subject == crate::data::MathSubject::Analysis)
            .filter(|e| category_key.as_ref().is_none_or(|k| e.topic.key() == *k))
            .filter(|e| progress.is_known(e.name))
            .count(),
        Subject::MathDataScience => MATH_ENTRIES
            .iter()
            .filter(|e| e.subject == crate::data::MathSubject::DataScience)
            .filter(|e| category_key.as_ref().is_none_or(|k| e.topic.key() == *k))
            .filter(|e| progress.is_known(e.name))
            .count(),
    }
}

#[derive(Clone, PartialEq)]
struct FilterState {
    include_unknown: bool,
    include_known: bool,
}

#[component]
pub fn CategoryScreen(
    subject: Subject,
    mut current_screen: Signal<Screen>,
    progress: Signal<Progress>,
) -> Element {
    let mut selected_category: Signal<Option<String>> = use_signal(|| None);
    let mut filters = use_signal(|| FilterState {
        include_unknown: true,
        include_known: false,
    });

    let categories = categories_for_subject(&subject);

    // Compute total/known/unknown for the selected category
    let subject_for_count = subject.clone();
    let word_count = use_memo(move || {
        let cat = selected_category.read().clone();
        count_entries(&subject_for_count, cat.as_deref())
    });

    let subject_for_known = subject.clone();
    let known_count = use_memo(move || {
        let cat = selected_category.read().clone();
        let prog = progress.read();
        count_known(&subject_for_known, cat.as_deref(), &prog)
    });

    let can_start = use_memo(move || word_count() >= 4);

    rsx! {
        div { class: "screen category-screen",
            // Header
            div { class: "screen-header",
                button {
                    class: "back-btn",
                    onclick: move |_| current_screen.set(Screen::Home),
                    "←"
                }
                h1 { class: "screen-title",
                    "{subject.flag()} {subject.display_name()}"
                }
            }

            div { class: "category-content",
                // Category picker
                h2 { class: "section-label", "Category" }
                div { class: "category-grid",
                    // "All" button
                    {
                        let total = count_entries(&subject, None);
                        let known = count_known(&subject, None, &progress.read());
                        rsx! {
                            button {
                                class: if selected_category.read().is_none() {
                                    "category-btn category-btn--active"
                                } else {
                                    "category-btn"
                                },
                                onclick: move |_| selected_category.set(None),
                                span { class: "category-emoji", "🌟" }
                                span { class: "category-name", "All" }
                                span { class: "category-count", "{known} / {total}" }
                            }
                        }
                    }
                    for cat in categories.iter() {
                        {
                            let cat = cat.clone();
                            let cat_key = cat.key.clone();
                            let cat_key_for_click = cat.key.clone();
                            let is_active = selected_category.read().as_ref() == Some(&cat.key);
                            let total = count_entries(&subject, Some(&cat_key));
                            let known = count_known(&subject, Some(&cat_key), &progress.read());
                            rsx! {
                                button {
                                    class: if is_active { "category-btn category-btn--active" } else { "category-btn" },
                                    onclick: move |_| selected_category.set(Some(cat_key_for_click.clone())),
                                    span { class: "category-emoji", "{cat.emoji}" }
                                    span { class: "category-name", "{cat.display_name}" }
                                    span { class: "category-count", "{known} / {total}" }
                                }
                            }
                        }
                    }
                }

                // Mastery progress bar for selected category
                {
                    let total = word_count();
                    let known = known_count();
                    let unknown = total - known;
                    let pct = if total > 0 { known * 100 / total } else { 0 };
                    rsx! {
                        div { class: "mastery-summary",
                            div { class: "mastery-bar-container",
                                div { class: "mastery-bar-fill", style: "width: {pct}%" }
                            }
                            div { class: "mastery-stats",
                                span { class: "mastery-stat mastery-stat--known", "✓ {known} mastered" }
                                span { class: "mastery-stat mastery-stat--unknown", "{unknown} to learn" }
                            }
                        }
                    }
                }

                // Filter checkboxes
                h2 { class: "section-label", "Include words" }
                div { class: "filter-group",
                    label { class: "filter-label",
                        input {
                            r#type: "checkbox",
                            class: "filter-checkbox",
                            checked: filters.read().include_unknown,
                            onchange: move |e| {
                                let checked = e.value() == "true";
                                filters.write().include_unknown = checked;
                                if !checked && !filters.read().include_known {
                                    filters.write().include_known = true;
                                }
                            }
                        }
                        span { "Unknown words" }
                        span { class: "filter-hint", "(not yet mastered)" }
                    }
                    label { class: "filter-label",
                        input {
                            r#type: "checkbox",
                            class: "filter-checkbox",
                            checked: filters.read().include_known,
                            onchange: move |e| {
                                let checked = e.value() == "true";
                                filters.write().include_known = checked;
                                if !checked && !filters.read().include_unknown {
                                    filters.write().include_unknown = true;
                                }
                            }
                        }
                        span { "Known words" }
                        span { class: "filter-hint", "(already mastered)" }
                    }
                }

                // Word count info
                div { class: "word-count-info",
                    if can_start() {
                        span { class: "word-count-ok",
                            "~{word_count()} words available"
                        }
                    } else {
                        span { class: "word-count-warn",
                            "Not enough words in this selection (min 4)"
                        }
                    }
                }
            }

            // Start button
            div { class: "start-section",
                button {
                    class: if can_start() { "start-btn" } else { "start-btn start-btn--disabled" },
                    disabled: !can_start(),
                    onclick: move |_| {
                        if can_start() {
                            let config = GameConfig {
                                subject: subject.clone(),
                                category: selected_category.read().clone(),
                                include_unknown: filters.read().include_unknown,
                                include_known: filters.read().include_known,
                                mode: GameMode::Words20,
                            };
                            current_screen.set(Screen::ModeSelect { config });
                        }
                    },
                    "Choose Mode  →"
                }
            }
        }
    }
}
