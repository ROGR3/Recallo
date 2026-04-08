use crate::data::units;
use crate::data::{Category, MathEntryType, MathSubject, MathTopic};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntryTypeFilter {
    Both,
    DefinitionsOnly,
    TheoremsOnly,
}

impl EntryTypeFilter {
    pub fn matches(self, entry_type: MathEntryType) -> bool {
        match self {
            EntryTypeFilter::Both => true,
            EntryTypeFilter::DefinitionsOnly => entry_type == MathEntryType::Definition,
            EntryTypeFilter::TheoremsOnly => entry_type == MathEntryType::Theorem,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Home,
    Settings,
    CategorySelect { subject: Subject },
    ModeSelect { config: GameConfig },
    Game { config: GameConfig },
    Results { result: GameResult },
    KnownWords { subject: Subject },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subject {
    Korean,
    MathAnalysis,
    MathDataScience,
}

impl Subject {
    pub fn display_name(self) -> &'static str {
        match self {
            Subject::Korean => "Korej\u{0161}tina",
            Subject::MathAnalysis => "Anal\u{00fd}za",
            Subject::MathDataScience => "Statistika",
        }
    }

    pub fn flag(self) -> &'static str {
        match self {
            Subject::Korean => "\u{1f1f0}\u{1f1f7}",
            Subject::MathAnalysis => "\u{1f4d0}",
            Subject::MathDataScience => "\u{1f4ca}",
        }
    }

    pub fn native_name(self) -> &'static str {
        match self {
            Subject::Korean => "\u{d55c}\u{ad6d}\u{c5b4}",
            Subject::MathAnalysis => "Anal\u{00fd}za",
            Subject::MathDataScience => "Statistika",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Subject::Korean => "korean",
            Subject::MathAnalysis => "math_analysis",
            Subject::MathDataScience => "math_datascience",
        }
    }

    pub fn is_math(self) -> bool {
        matches!(self, Subject::MathAnalysis | Subject::MathDataScience)
    }

    /// Returns the `MathSubject` for math subjects, or `None` for Korean.
    pub fn math_subject(self) -> Option<MathSubject> {
        match self {
            Subject::Korean => None,
            Subject::MathAnalysis => Some(MathSubject::Analysis),
            Subject::MathDataScience => Some(MathSubject::DataScience),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameMode {
    /// 10 words, +30s penalty per mistake
    Words10,
    /// 20 words, +30s penalty per mistake
    Words20,
}

impl GameMode {
    pub fn word_count(self) -> usize {
        match self {
            GameMode::Words10 => 10,
            GameMode::Words20 => 20,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            GameMode::Words10 => "10 ot\u{00e1}zek",
            GameMode::Words20 => "20 ot\u{00e1}zek",
        }
    }

    pub fn subtitle(self) -> &'static str {
        match self {
            GameMode::Words10 => "Rychl\u{00e9} kolo",
            GameMode::Words20 => "Standardn\u{00ed} kolo",
        }
    }

    pub fn emoji(self) -> &'static str {
        match self {
            GameMode::Words10 => "\u{26a1}",
            GameMode::Words20 => "\u{1f4dd}",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            GameMode::Words10 => "10",
            GameMode::Words20 => "20",
        }
    }
}

/// Seconds added to the timer for each wrong answer.
pub const PENALTY_SECONDS: f64 = 30.0;

#[derive(Debug, Clone, PartialEq)]
pub struct GameConfig {
    pub subject: Subject,
    /// None = "All categories"; Some(key) = specific category/topic key
    pub category: Option<String>,
    pub include_unknown: bool,
    pub include_known: bool,
    pub mode: GameMode,
    pub entry_type_filter: EntryTypeFilter,
    pub hard_mode: bool,
}

impl GameConfig {
    pub fn category_key(&self) -> String {
        match &self.category {
            Some(c) => c.clone(),
            None => "all".to_string(),
        }
    }

    /// Whether a word with the given known status should be included per filter settings.
    pub fn should_include(&self, is_known: bool) -> bool {
        match (self.include_unknown, self.include_known) {
            (true, true) | (false, false) => true,
            (true, false) => !is_known,
            (false, true) => is_known,
        }
    }

    /// Resolve category key to a human-readable display name.
    pub fn category_display_name(&self) -> String {
        match &self.category {
            None => "V\u{0161}e".to_string(),
            Some(key) => {
                if let Some((sec, unit)) = units::parse_unit_key(key) {
                    return units::get_unit(sec, unit)
                        .map(|u| format!("S{}\u{00b7}{}", u.section, u.name))
                        .unwrap_or_else(|| key.clone());
                }
                if let Some(ms) = self.subject.math_subject() {
                    return MathTopic::all_for_subject(ms)
                        .iter()
                        .find(|t| t.key() == key.as_str())
                        .map(|t| t.display_name().to_string())
                        .unwrap_or_else(|| key.clone());
                }
                // Korean category
                Category::all()
                    .iter()
                    .find(|c| c.display_name().to_lowercase() == *key)
                    .map(|c| c.display_name().to_string())
                    .unwrap_or_else(|| key.clone())
            }
        }
    }

    pub fn best_time_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.subject.key(),
            self.category_key(),
            self.mode.key()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameResult {
    pub config: GameConfig,
    pub score: usize,
    pub total: usize,
    pub time_seconds: f64,
    pub penalty_seconds: f64,
    pub mistakes: usize,
    pub is_new_best: bool,
    pub previous_best: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl Theme {
    pub fn css_class(self) -> &'static str {
        match self {
            Theme::System => "",
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Theme::System => "Syst\u{00e9}m",
            Theme::Light => "Sv\u{011b}tl\u{00fd}",
            Theme::Dark => "Tmav\u{00fd}",
        }
    }

    pub fn emoji(self) -> &'static str {
        match self {
            Theme::System => "\u{1f4f1}",
            Theme::Light => "\u{2600}\u{fe0f}",
            Theme::Dark => "\u{1f319}",
        }
    }
}
