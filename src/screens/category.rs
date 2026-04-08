use crate::data::{self, units};
use crate::state::{EntryTypeFilter, GameConfig, GameMode, Progress, Screen, Subject};
use crate::util::{go_back, navigate};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    ByType,
    BySection,
}

#[derive(Clone, PartialEq)]
struct FilterState {
    include_unknown: bool,
    include_known: bool,
}

/// Minimum number of words required to start a game (needed for MC distractors).
const MIN_WORDS_FOR_GAME: usize = 4;

#[component]
pub fn CategoryScreen(
    subject: Subject,
    mut current_screen: Signal<Screen>,
    mut history: Signal<Vec<Screen>>,
    progress: Signal<Progress>,
) -> Element {
    let mut selected_category: Signal<Option<String>> = use_signal(|| None);
    let mut filters = use_signal(|| FilterState {
        include_unknown: true,
        include_known: false,
    });
    let mut view_mode = use_signal(|| {
        if subject == Subject::Korean {
            ViewMode::BySection
        } else {
            ViewMode::ByType
        }
    });
    let mut entry_type_filter: Signal<EntryTypeFilter> =
        use_signal(|| EntryTypeFilter::Both);
    let mut hard_mode: Signal<bool> = use_signal(|| true);

    let categories = data::categories_for_subject(subject);
    let is_korean = subject == Subject::Korean;
    let is_math = subject.is_math();

    let word_count = use_memo(move || {
        let cat = selected_category.read().clone();
        let etf = *entry_type_filter.read();
        data::count_entries(subject, cat.as_deref(), etf)
    });

    let known_count = use_memo(move || {
        let cat = selected_category.read().clone();
        let etf = *entry_type_filter.read();
        let prog = progress.read();
        data::count_known(subject, cat.as_deref(), etf, &prog)
    });

    let can_start = use_memo(move || word_count() >= MIN_WORDS_FOR_GAME);

    rsx! {
        div { class: "screen category-screen",
            div { class: "screen-header",
                button {
                    class: "back-btn",
                    onclick: move |_| go_back(&mut current_screen, &mut history),
                    "\u{2190}"
                }
                h1 { class: "screen-title",
                    "{subject.flag()} {subject.display_name()}"
                }
            }

            div { class: "category-content",
                // View mode toggle (Korean only)
                if is_korean {
                    div { class: "view-toggle",
                        button {
                            class: if *view_mode.read() == ViewMode::ByType { "view-toggle-btn view-toggle-btn--active" } else { "view-toggle-btn" },
                            onclick: move |_| {
                                view_mode.set(ViewMode::ByType);
                                selected_category.set(None);
                            },
                            "Dle typu"
                        }
                        button {
                            class: if *view_mode.read() == ViewMode::BySection { "view-toggle-btn view-toggle-btn--active" } else { "view-toggle-btn" },
                            onclick: move |_| {
                                view_mode.set(ViewMode::BySection);
                                selected_category.set(None);
                            },
                            "Dle sekce"
                        }
                    }
                }

                // Entry type filter + hard mode (math subjects only)
                if is_math {
                    div { class: "view-toggle",
                        button {
                            class: if *entry_type_filter.read() == EntryTypeFilter::Both { "view-toggle-btn view-toggle-btn--active" } else { "view-toggle-btn" },
                            onclick: move |_| entry_type_filter.set(EntryTypeFilter::Both),
                            "V\u{0161}e"
                        }
                        button {
                            class: if *entry_type_filter.read() == EntryTypeFilter::DefinitionsOnly { "view-toggle-btn view-toggle-btn--active" } else { "view-toggle-btn" },
                            onclick: move |_| entry_type_filter.set(EntryTypeFilter::DefinitionsOnly),
                            "Definice"
                        }
                        button {
                            class: if *entry_type_filter.read() == EntryTypeFilter::TheoremsOnly { "view-toggle-btn view-toggle-btn--active" } else { "view-toggle-btn" },
                            onclick: move |_| entry_type_filter.set(EntryTypeFilter::TheoremsOnly),
                            "V\u{011b}ty"
                        }
                    }
                    div { class: "view-toggle",
                        button {
                            class: if *hard_mode.read() { "view-toggle-btn view-toggle-btn--active" } else { "view-toggle-btn" },
                            onclick: move |_| hard_mode.set(true),
                            "Hard"
                        }
                        button {
                            class: if !*hard_mode.read() { "view-toggle-btn view-toggle-btn--active" } else { "view-toggle-btn" },
                            onclick: move |_| hard_mode.set(false),
                            "Easy"
                        }
                    }
                }

                if *view_mode.read() == ViewMode::BySection && is_korean {
                    {render_section_view(subject, selected_category, progress)}
                } else {
                    h2 { class: "section-label", "Kategorie" }
                    div { class: "category-grid",
                        {
                            let etf = *entry_type_filter.read();
                            let total = data::count_entries(subject, None, etf);
                            let known = data::count_known(subject, None, etf, &progress.read());
                            rsx! {
                                button {
                                    class: if selected_category.read().is_none() {
                                        "category-btn category-btn--active"
                                    } else {
                                        "category-btn"
                                    },
                                    onclick: move |_| selected_category.set(None),
                                    span { class: "category-emoji", "\u{1f31f}" }
                                    span { class: "category-name", "V\u{0161}e" }
                                    span { class: "category-count", "{known} / {total}" }
                                }
                            }
                        }
                        for cat in categories.iter() {
                            {
                                let key = cat.key.clone();
                                let key_click = cat.key.clone();
                                let is_active = selected_category.read().as_ref() == Some(&cat.key);
                                let etf = *entry_type_filter.read();
                                let total = data::count_entries(subject, Some(&key), etf);
                                let known = data::count_known(subject, Some(&key), etf, &progress.read());
                                rsx! {
                                    button {
                                        class: if is_active { "category-btn category-btn--active" } else { "category-btn" },
                                        onclick: move |_| selected_category.set(Some(key_click.clone())),
                                        span { class: "category-emoji", "{cat.emoji}" }
                                        span { class: "category-name", "{cat.display_name}" }
                                        span { class: "category-count", "{known} / {total}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Mastery progress bar
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
                                span { class: "mastery-stat mastery-stat--known", "\u{2713} {known} um\u{00ed}m" }
                                span { class: "mastery-stat mastery-stat--unknown", "{unknown} zb\u{00fd}v\u{00e1}" }
                            }
                            if known > 0 {
                                button {
                                    class: "mastery-view-btn",
                                    onclick: move |_| {
                                        navigate(
                                            &mut current_screen,
                                            &mut history,
                                            Screen::KnownWords { subject },
                                        );
                                    },
                                    "Zobrazit nau\u{010d}en\u{00e9} \u{2192}"
                                }
                            }
                        }
                    }
                }

                // Filter checkboxes
                h2 { class: "section-label", "Zahrnout" }
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
                        span { "Nenau\u{010d}en\u{00e9}" }
                        span { class: "filter-hint", "(je\u{0161}t\u{011b} neum\u{00ed}m)" }
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
                        span { "Nau\u{010d}en\u{00e9}" }
                        span { class: "filter-hint", "(u\u{017e} um\u{00ed}m)" }
                    }
                }

                div { class: "word-count-info",
                    if can_start() {
                        span { class: "word-count-ok",
                            "~{word_count()} ot\u{00e1}zek k dispozici"
                        }
                    } else {
                        span { class: "word-count-warn",
                            "Nedostatek ot\u{00e1}zek (min. {MIN_WORDS_FOR_GAME})"
                        }
                    }
                }
            }

            div { class: "start-section",
                button {
                    class: if can_start() { "start-btn" } else { "start-btn start-btn--disabled" },
                    disabled: !can_start(),
                    onclick: move |_| {
                        if can_start() {
                            let config = GameConfig {
                                subject,
                                category: selected_category.read().clone(),
                                include_unknown: filters.read().include_unknown,
                                include_known: filters.read().include_known,
                                mode: GameMode::Words20,
                                entry_type_filter: *entry_type_filter.read(),
                                hard_mode: *hard_mode.read(),
                            };
                            navigate(&mut current_screen, &mut history, Screen::ModeSelect { config });
                        }
                    },
                    "Zvolit re\u{017e}im  \u{2192}"
                }
            }
        }
    }
}

fn render_section_view(
    subject: Subject,
    mut selected_category: Signal<Option<String>>,
    progress: Signal<Progress>,
) -> Element {
    let sections = units::all_sections();
    rsx! {
        div { class: "category-grid",
            {
                let total = data::count_entries(subject, None, EntryTypeFilter::Both);
                let known = data::count_known(subject, None, EntryTypeFilter::Both, &progress.read());
                rsx! {
                    button {
                        class: if selected_category.read().is_none() {
                            "category-btn category-btn--active"
                        } else {
                            "category-btn"
                        },
                        onclick: move |_| selected_category.set(None),
                        span { class: "category-emoji", "\u{1f31f}" }
                        span { class: "category-name", "V\u{0161}e" }
                        span { class: "category-count", "{known} / {total}" }
                    }
                }
            }
        }
        for &sec in sections.iter() {
            {
                let section_units = units::units_in_section(sec);
                let sec_name = units::section_name(sec);
                rsx! {
                    h2 { class: "section-label section-label--unit", "{sec_name}" }
                    div { class: "category-grid category-grid--units",
                        for u in section_units.iter() {
                            {
                                let key = units::unit_key(u.section, u.unit);
                                let key_click = key.clone();
                                let is_active = selected_category.read().as_ref() == Some(&key);
                                let total = data::count_entries(subject, Some(&key), EntryTypeFilter::Both);
                                let known = data::count_known(subject, Some(&key), EntryTypeFilter::Both, &progress.read());
                                rsx! {
                                    button {
                                        class: if is_active { "unit-btn unit-btn--active" } else { "unit-btn" },
                                        onclick: move |_| selected_category.set(Some(key_click.clone())),
                                        div { class: "unit-btn-top",
                                            span { class: "unit-emoji", "{u.emoji}" }
                                            span { class: "unit-number", "{u.unit}" }
                                        }
                                        span { class: "unit-name", "{u.name}" }
                                        span { class: "unit-count", "{known}/{total}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
