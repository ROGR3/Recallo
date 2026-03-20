use crate::data::Category;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Home,
    CategorySelect { language: Language },
    ModeSelect { config: GameConfig },
    Game { config: GameConfig },
    Results { result: GameResult },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Korean,
}

impl Language {
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Korean => "Korean",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            Language::Korean => "🇰🇷",
        }
    }

    pub fn native_name(&self) -> &'static str {
        match self {
            Language::Korean => "한국어",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Language::Korean => "korean",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameMode {
    /// Standard 10 words — mistakes allowed
    Normal10,
    /// Standard 20 words — mistakes allowed
    Normal20,
    /// 10 words — game ends on first mistake
    Perfect10,
    /// 20 words — game ends on first mistake
    Perfect20,
}

impl GameMode {
    pub fn word_count(&self) -> usize {
        match self {
            GameMode::Normal10 | GameMode::Perfect10 => 10,
            GameMode::Normal20 | GameMode::Perfect20 => 20,
        }
    }

    pub fn is_no_mistakes(&self) -> bool {
        matches!(self, GameMode::Perfect10 | GameMode::Perfect20)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            GameMode::Normal10 => "10 Words",
            GameMode::Normal20 => "20 Words",
            GameMode::Perfect10 => "10 Words (Perfect)",
            GameMode::Perfect20 => "20 Words (Perfect)",
        }
    }

    pub fn subtitle(&self) -> &'static str {
        match self {
            GameMode::Normal10 => "Quick round",
            GameMode::Normal20 => "Standard round",
            GameMode::Perfect10 => "No mistakes allowed",
            GameMode::Perfect20 => "No mistakes allowed",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            GameMode::Normal10 => "⚡",
            GameMode::Normal20 => "📝",
            GameMode::Perfect10 => "💎",
            GameMode::Perfect20 => "🔥",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            GameMode::Normal10 => "normal10",
            GameMode::Normal20 => "normal20",
            GameMode::Perfect10 => "perfect10",
            GameMode::Perfect20 => "perfect20",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameConfig {
    pub language: Language,
    /// None = "All categories"
    pub category: Option<Category>,
    pub include_unknown: bool,
    pub include_known: bool,
    pub mode: GameMode,
}

impl GameConfig {
    pub fn category_key(&self) -> String {
        match &self.category {
            Some(c) => c.display_name().to_lowercase(),
            None => "all".to_string(),
        }
    }

    pub fn best_time_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.language.key(),
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
    pub is_new_best: bool,
    pub previous_best: Option<f64>,
    /// true if game ended early due to a mistake in perfect mode
    pub failed_perfect: bool,
}
