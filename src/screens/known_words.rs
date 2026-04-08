use crate::data::{Category, MathSubject, MathTopic, MATH_ENTRIES, WORDS};
use crate::state::{save_progress, Progress, Screen, Subject};
use crate::util::go_back;
use dioxus::prelude::*;

/// A known entry with display strings and a key for unmarking.
struct KnownEntry {
    key: String,
    primary: String,
    secondary: String,
}

/// Group of known entries under a category heading.
struct KnownGroup {
    emoji: &'static str,
    name: &'static str,
    entries: Vec<KnownEntry>,
}

fn build_korean_groups(progress: &Progress) -> Vec<KnownGroup> {
    Category::all()
        .iter()
        .filter_map(|cat| {
            let entries: Vec<KnownEntry> = WORDS
                .iter()
                .filter(|w| w.category == *cat && progress.is_known(w.korean))
                .map(|w| KnownEntry {
                    key: w.korean.to_string(),
                    primary: w.korean.to_string(),
                    secondary: w.english.to_string(),
                })
                .collect();
            if entries.is_empty() {
                return None;
            }
            Some(KnownGroup {
                emoji: cat.emoji(),
                name: cat.display_name(),
                entries,
            })
        })
        .collect()
}

fn build_math_groups(progress: &Progress, subject: MathSubject) -> Vec<KnownGroup> {
    MathTopic::all_for_subject(subject)
        .iter()
        .filter_map(|topic| {
            let entries: Vec<KnownEntry> = MATH_ENTRIES
                .iter()
                .filter(|e| e.topic == *topic && e.subject == subject && progress.is_known(e.name))
                .map(|e| KnownEntry {
                    key: e.name.to_string(),
                    primary: e.name.to_string(),
                    secondary: e.entry_type.display_name().to_string(),
                })
                .collect();
            if entries.is_empty() {
                return None;
            }
            Some(KnownGroup {
                emoji: topic.emoji(),
                name: topic.display_name(),
                entries,
            })
        })
        .collect()
}

fn build_groups(subject: Subject, progress: &Progress) -> Vec<KnownGroup> {
    match subject.math_subject() {
        Some(ms) => build_math_groups(progress, ms),
        None => build_korean_groups(progress),
    }
}

#[component]
pub fn KnownWordsScreen(
    subject: Subject,
    mut current_screen: Signal<Screen>,
    mut history: Signal<Vec<Screen>>,
    mut progress: Signal<Progress>,
) -> Element {
    let groups = build_groups(subject, &progress.read());
    let total_known: usize = groups.iter().map(|g| g.entries.len()).sum();

    rsx! {
        div { class: "screen category-screen",
            div { class: "screen-header",
                button {
                    class: "back-btn",
                    onclick: move |_| go_back(&mut current_screen, &mut history),
                    "\u{2190}"
                }
                h1 { class: "screen-title",
                    "{subject.flag()} Nau\u{010d}en\u{00e9}"
                }
            }

            div { class: "category-content",
                if total_known == 0 {
                    div { class: "empty-state",
                        span { style: "font-size: 2.4rem;", "\u{1f331}" }
                        p { "Zat\u{00ed}m \u{017e}\u{00e1}dn\u{00e9} nau\u{010d}en\u{00e9}." }
                        p { style: "font-size: 0.85rem; color: var(--text-3);",
                            "Zahraj si kv\u{00ed}z a klikni na \"Toto u\u{017e} um\u{00ed}m\"."
                        }
                    }
                } else {
                    {
                        let label = format!("{total_known} nau\u{010d}eno");
                        rsx! {
                            div { class: "known-summary-pill", "{label}" }
                        }
                    }

                    for group in groups.iter() {
                        div { class: "known-group",
                            h2 { class: "section-label",
                                "{group.emoji} {group.name} ({group.entries.len()})"
                            }
                            div { class: "known-list",
                                for entry in group.entries.iter() {
                                    {
                                        let key = entry.key.clone();
                                        rsx! {
                                            div { class: "known-item",
                                                div { class: "known-item-text",
                                                    span { class: "known-item-primary", "{entry.primary}" }
                                                    span { class: "known-item-secondary", "{entry.secondary}" }
                                                }
                                                button {
                                                    class: "known-item-remove",
                                                    onclick: move |_| {
                                                        progress.write().unmark_known(&key);
                                                        save_progress(&progress.read());
                                                    },
                                                    "\u{00d7}"
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
        }
    }
}
