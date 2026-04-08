pub mod math_words;
pub mod units;
pub mod words;

pub use math_words::{MathEntry, MathEntryType, MathSubject, MathTopic, MATH_ENTRIES};
pub use words::{Category, Word, WORDS};

use crate::state::{Progress, Subject};

// ── Shared entry helpers ────────────────────────────────────────────────────

/// Check whether a Korean word matches a category key (type or unit).
pub fn word_matches_category(word: &Word, cat_key: Option<&str>) -> bool {
    match cat_key {
        None => true,
        Some(k) => {
            if let Some((sec, unit)) = units::parse_unit_key(k) {
                units::word_unit(word.korean) == Some((sec, unit))
            } else {
                word.category.display_name().to_lowercase() == k
            }
        }
    }
}

/// Category info for display (emoji, name, key) — used by category and known-words screens.
pub struct CategoryInfo {
    pub key: String,
    pub display_name: &'static str,
    pub emoji: &'static str,
}

/// Return the list of categories/topics for a given subject.
pub fn categories_for_subject(subject: Subject) -> Vec<CategoryInfo> {
    if let Some(ms) = subject.math_subject() {
        MathTopic::all_for_subject(ms)
            .iter()
            .map(|t| CategoryInfo {
                key: t.key().to_string(),
                display_name: t.display_name(),
                emoji: t.emoji(),
            })
            .collect()
    } else {
        Category::all()
            .iter()
            .map(|c| CategoryInfo {
                key: c.display_name().to_lowercase(),
                display_name: c.display_name(),
                emoji: c.emoji(),
            })
            .collect()
    }
}

/// Count total entries for a subject and optional category.
pub fn count_entries(subject: Subject, category_key: Option<&str>) -> usize {
    if let Some(ms) = subject.math_subject() {
        MATH_ENTRIES
            .iter()
            .filter(|e| e.subject == ms)
            .filter(|e| category_key.is_none_or(|k| e.topic.key() == k))
            .count()
    } else {
        WORDS
            .iter()
            .filter(|w| word_matches_category(w, category_key))
            .count()
    }
}

/// Count known entries for a subject and optional category.
pub fn count_known(subject: Subject, category_key: Option<&str>, progress: &Progress) -> usize {
    if let Some(ms) = subject.math_subject() {
        MATH_ENTRIES
            .iter()
            .filter(|e| e.subject == ms)
            .filter(|e| category_key.is_none_or(|k| e.topic.key() == k))
            .filter(|e| progress.is_known(e.name))
            .count()
    } else {
        WORDS
            .iter()
            .filter(|w| word_matches_category(w, category_key))
            .filter(|w| progress.is_known(w.korean))
            .count()
    }
}
