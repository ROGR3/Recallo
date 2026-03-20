mod data;
mod screens;
mod state;

use dioxus::prelude::*;
use screens::{CategoryScreen, GameScreen, HomeScreen, ModeSelectScreen, ResultsScreen};
use state::{load_progress, Progress, Screen};

const STYLES: Asset = asset!("/assets/main.css");
pub const LOGO: Asset = asset!("/assets/logo.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let current_screen: Signal<Screen> = use_signal(|| Screen::Home);
    let progress: Signal<Progress> = use_signal(load_progress);

    rsx! {
        document::Stylesheet { href: STYLES }
        div { class: "app",
            match current_screen.read().clone() {
                Screen::Home => rsx! {
                    HomeScreen { current_screen }
                },
                Screen::CategorySelect { language } => rsx! {
                    CategoryScreen {
                        language,
                        current_screen,
                        progress,
                    }
                },
                Screen::ModeSelect { config } => rsx! {
                    ModeSelectScreen {
                        config,
                        current_screen,
                        progress,
                    }
                },
                Screen::Game { config } => rsx! {
                    GameScreen {
                        config,
                        current_screen,
                        progress,
                    }
                },
                Screen::Results { result } => rsx! {
                    ResultsScreen {
                        result,
                        current_screen,
                    }
                },
            }
        }
    }
}
