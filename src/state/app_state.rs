use crate::data::{Category, MathTopic};
use crate::data::units;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Home,
    Settings,
    CategorySelect { subject: Subject },
    ModeSelect { config: GameConfig },
    Game { config: GameConfig },
    Results { result: GameResult },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subject {
    Korean,
    MathAnalysis,
    MathDataScience,
}

impl Subject {
    pub fn display_name(&self) -> &'static str {
        match self {
            Subject::Korean => "Korean",
            Subject::MathAnalysis => "Math Analysis",
            Subject::MathDataScience => "Math DataScience",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            Subject::Korean => "🇰🇷",
            Subject::MathAnalysis => "📐",
            Subject::MathDataScience => "📊",
        }
    }

    pub fn native_name(&self) -> &'static str {
        match self {
            Subject::Korean => "한국어",
            Subject::MathAnalysis => "Analýza",
            Subject::MathDataScience => "Statistika",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Subject::Korean => "korean",
            Subject::MathAnalysis => "math_analysis",
            Subject::MathDataScience => "math_datascience",
        }
    }

    pub fn is_math(&self) -> bool {
        matches!(self, Subject::MathAnalysis | Subject::MathDataScience)
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
    pub fn word_count(&self) -> usize {
        match self {
            GameMode::Words10 => 10,
            GameMode::Words20 => 20,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            GameMode::Words10 => "10 Words",
            GameMode::Words20 => "20 Words",
        }
    }

    pub fn subtitle(&self) -> &'static str {
        match self {
            GameMode::Words10 => "Quick round",
            GameMode::Words20 => "Standard round",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            GameMode::Words10 => "⚡",
            GameMode::Words20 => "📝",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            GameMode::Words10 => "10",
            GameMode::Words20 => "20",
        }
    }
}

/// Seconds added to the timer for each wrong answer
pub const PENALTY_SECONDS: f64 = 30.0;

#[derive(Debug, Clone, PartialEq)]
pub struct GameConfig {
    pub subject: Subject,
    /// None = "All categories"; Some(key) = specific category/topic key
    pub category: Option<String>,
    pub include_unknown: bool,
    pub include_known: bool,
    pub mode: GameMode,
}

impl GameConfig {
    pub fn category_key(&self) -> String {
        match &self.category {
            Some(c) => c.clone(),
            None => "all".to_string(),
        }
    }

    /// Resolve category key to a human-readable display name.
    pub fn category_display_name(&self) -> String {
        match &self.category {
            None => "All".to_string(),
            Some(key) => match &self.subject {
                Subject::Korean => {
                    if let Some((sec, unit)) = units::parse_unit_key(key) {
                        units::get_unit(sec, unit)
                            .map(|u| format!("S{}·{}", u.section, u.name))
                            .unwrap_or_else(|| key.clone())
                    } else {
                        Category::all()
                            .iter()
                            .find(|c| c.display_name().to_lowercase() == *key)
                            .map(|c| c.display_name().to_string())
                            .unwrap_or_else(|| key.clone())
                    }
                }
                Subject::MathAnalysis => {
                    MathTopic::all_for_subject(crate::data::MathSubject::Analysis)
                        .into_iter()
                        .find(|t| t.key() == key)
                        .map(|t| t.display_name().to_string())
                        .unwrap_or_else(|| key.clone())
                }
                Subject::MathDataScience => {
                    MathTopic::all_for_subject(crate::data::MathSubject::DataScience)
                        .into_iter()
                        .find(|t| t.key() == key)
                        .map(|t| t.display_name().to_string())
                        .unwrap_or_else(|| key.clone())
                }
            },
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
    pub fn css_class(&self) -> &'static str {
        match self {
            Theme::System => "",
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Theme::System => "System",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        }
    }
}
