use crate::data::{Category, MathSubject, MathTopic, MATH_ENTRIES, WORDS};
use crate::state::{save_progress, Progress, Screen, Subject};
use dioxus::prelude::*;

fn go_back(current_screen: &mut Signal<Screen>, history: &mut Signal<Vec<Screen>>) {
    if let Some(prev) = history.write().pop() {
        current_screen.set(prev);
    } else {
        current_screen.set(Screen::Home);
    }
}

/// A known entry with its display strings and key for unmarking.
struct KnownEntry {
    key: String,
    primary: String,
    secondary: String,
}

/// Group of known entries under a heading.
struct KnownGroup {
    emoji: String,
    name: String,
    entries: Vec<KnownEntry>,
}

fn build_korean_groups(progress: &Progress) -> Vec<KnownGroup> {
    let mut groups: Vec<KnownGroup> = Category::all()
        .iter()
        .map(|cat| {
            let entries: Vec<KnownEntry> = WORDS
                .iter()
                .filter(|w| w.category == *cat && progress.is_known(w.korean))
                .map(|w| KnownEntry {
                    key: w.korean.to_string(),
                    primary: w.korean.to_string(),
                    secondary: w.english.to_string(),
                })
                .collect();
            KnownGroup {
                emoji: cat.emoji().to_string(),
                name: cat.display_name().to_string(),
                entries,
            }
        })
        .filter(|g| !g.entries.is_empty())
        .collect();
    groups.sort_by(|a, b| b.entries.len().cmp(&a.entries.len()));
    groups
}

fn build_math_groups(progress: &Progress, subject: MathSubject) -> Vec<KnownGroup> {
    let topics = MathTopic::all_for_subject(subject);
    let mut groups: Vec<KnownGroup> = topics
        .into_iter()
        .map(|topic| {
            let entries: Vec<KnownEntry> = MATH_ENTRIES
                .iter()
                .filter(|e| e.topic == topic && e.subject == subject && progress.is_known(e.name))
                .map(|e| KnownEntry {
                    key: e.name.to_string(),
                    primary: e.name.to_string(),
                    secondary: e.entry_type.display_name().to_string(),
                })
                .collect();
            KnownGroup {
                emoji: topic.emoji().to_string(),
                name: topic.display_name().to_string(),
                entries,
            }
        })
        .filter(|g| !g.entries.is_empty())
        .collect();
    groups.sort_by(|a, b| b.entries.len().cmp(&a.entries.len()));
    groups
}

#[component]
pub fn KnownWordsScreen(
    subject: Subject,
    mut current_screen: Signal<Screen>,
    mut history: Signal<Vec<Screen>>,
    mut progress: Signal<Progress>,
) -> Element {
    let groups = {
        let prog = progress.read();
        match &subject {
            Subject::Korean => build_korean_groups(&prog),
            Subject::MathAnalysis => build_math_groups(&prog, MathSubject::Analysis),
            Subject::MathDataScience => build_math_groups(&prog, MathSubject::DataScience),
        }
    };

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
                    "{subject.flag()} Mastered Words"
                }
            }

            div { class: "category-content",
                if total_known == 0 {
                    div { class: "empty-state",
                        span { style: "font-size: 2.4rem;", "\u{1F331}" }
                        p { "No mastered words yet." }
                        p { style: "font-size: 0.85rem; color: var(--text-3);",
                            "Play a quiz and tap \"I know this\" to mark words."
                        }
                    }
                } else {
                    // Summary pill
                    {
                        let label = if total_known == 1 {
                            format!("{} word mastered", total_known)
                        } else {
                            format!("{} words mastered", total_known)
                        };
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
