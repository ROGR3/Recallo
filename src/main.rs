mod data;
mod screens;
mod state;

use dioxus::prelude::*;
use screens::{
    CategoryScreen, GameScreen, HomeScreen, ModeSelectScreen, ResultsScreen, SettingsScreen,
};
use state::{load_progress, Progress, Screen};

const STYLES: Asset = asset!("/assets/main.css");
pub const LOGO: Asset = asset!("/assets/logo.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut current_screen: Signal<Screen> = use_signal(|| Screen::Home);
    let progress: Signal<Progress> = use_signal(load_progress);
    let mut history: Signal<Vec<Screen>> = use_signal(Vec::new);
    // Flag to distinguish back-navigation from forward-navigation
    let mut is_back_nav: Signal<bool> = use_signal(|| false);

    // Whenever screen changes (forward navigation), push browser history state
    use_effect(move || {
        // Subscribe to screen changes
        let _screen = current_screen.read().clone();
        // If this was triggered by a back-nav, don't push
        if *is_back_nav.peek() {
            is_back_nav.set(false);
            return;
        }
        // Push a browser history entry so Android back button triggers popstate
        spawn(async move {
            _ = document::eval(r#"
                if (window.__recallo_history_depth === undefined) {
                    window.__recallo_history_depth = 0;
                }
                window.__recallo_history_depth++;
                history.pushState({depth: window.__recallo_history_depth}, '');
            "#);
        });
    });

    // Listen for popstate (Android back button) and navigate back
    use_effect(move || {
        spawn(async move {
            // Set up the popstate listener once
            let mut eval = document::eval(r#"
                // Return a promise that resolves each time popstate fires
                await new Promise((resolve) => {
                    window.addEventListener('popstate', function handler(e) {
                        resolve('back');
                        window.removeEventListener('popstate', handler);
                    });
                });
            "#);
            loop {
                if let Ok(_val) = eval.recv::<String>().await {
                    let hist = history.read().clone();
                    if let Some(prev_screen) = hist.last().cloned() {
                        // Pop history and go back
                        history.write().pop();
                        is_back_nav.set(true);
                        current_screen.set(prev_screen);
                    }
                    // Re-register listener for next back press
                    eval = document::eval(r#"
                        await new Promise((resolve) => {
                            window.addEventListener('popstate', function handler(e) {
                                resolve('back');
                                window.removeEventListener('popstate', handler);
                            });
                        });
                    "#);
                }
            }
        });
    });

    let theme_class = progress.read().theme.css_class();

    rsx! {
        document::Stylesheet { href: STYLES }
        div { class: "app {theme_class}",
            match current_screen.read().clone() {
                Screen::Home => rsx! {
                    HomeScreen { current_screen, history }
                },
                Screen::Settings => rsx! {
                    SettingsScreen {
                        current_screen,
                        history,
                        progress,
                    }
                },
                Screen::CategorySelect { subject } => rsx! {
                    CategoryScreen {
                        subject,
                        current_screen,
                        history,
                        progress,
                    }
                },
                Screen::ModeSelect { config } => rsx! {
                    ModeSelectScreen {
                        config,
                        current_screen,
                        history,
                        progress,
                    }
                },
                Screen::Game { config } => rsx! {
                    GameScreen {
                        config,
                        current_screen,
                        history,
                        progress,
                    }
                },
                Screen::Results { result } => rsx! {
                    ResultsScreen {
                        result,
                        current_screen,
                        history,
                    }
                },
            }
        }
    }
}
