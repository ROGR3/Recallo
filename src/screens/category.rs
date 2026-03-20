use crate::data::{Category, WORDS};
use crate::state::{GameConfig, GameMode, Language, Progress, Screen};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct FilterState {
    include_unknown: bool,
    include_known: bool,
}

#[component]
pub fn CategoryScreen(
    language: Language,
    mut current_screen: Signal<Screen>,
    progress: Signal<Progress>,
) -> Element {
    let mut selected_category: Signal<Option<Category>> = use_signal(|| None);
    let mut filters = use_signal(|| FilterState {
        include_unknown: true,
        include_known: false,
    });

    // Compute total/known/unknown for the selected category
    let word_count = use_memo(move || {
        let cat = selected_category.read().clone();
        WORDS
            .iter()
            .filter(|w| cat.as_ref().is_none_or(|c| &w.category == c))
            .count()
    });

    let known_count = use_memo(move || {
        let cat = selected_category.read().clone();
        let prog = progress.read();
        WORDS
            .iter()
            .filter(|w| cat.as_ref().is_none_or(|c| &w.category == c))
            .filter(|w| prog.is_known(w.korean))
            .count()
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
                    "{language.flag()} {language.display_name()}"
                }
            }

            div { class: "category-content",
                // Category picker
                h2 { class: "section-label", "Category" }
                div { class: "category-grid",
                    // "All" button
                    {
                        let total = WORDS.len();
                        let known = WORDS.iter().filter(|w| progress.read().is_known(w.korean)).count();
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
                    for cat in Category::all() {
                        {
                            let cat = cat.clone();
                            let cat_for_click = cat.clone();
                            let is_active = selected_category.read().as_ref() == Some(&cat);
                            let cat_words: Vec<_> = WORDS.iter().filter(|w| w.category == cat).collect();
                            let total = cat_words.len();
                            let known = cat_words.iter().filter(|w| progress.read().is_known(w.korean)).count();
                            rsx! {
                                button {
                                    class: if is_active { "category-btn category-btn--active" } else { "category-btn" },
                                    onclick: move |_| selected_category.set(Some(cat_for_click.clone())),
                                    span { class: "category-emoji", "{cat.emoji()}" }
                                    span { class: "category-name", "{cat.display_name()}" }
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
                                language: language.clone(),
                                category: selected_category.read().clone(),
                                include_unknown: filters.read().include_unknown,
                                include_known: filters.read().include_known,
                                mode: GameMode::Normal20,
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
