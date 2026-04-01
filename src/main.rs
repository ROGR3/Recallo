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

    // Install a persistent popstate listener once, and poll for back-button
    // events in a loop. Each popstate increments a JS counter; the Rust side
    // compares its own counter and pops the app history stack when they diverge.
    use_effect(move || {
        spawn(async move {
            // One-time setup: install the persistent listener and seed browser
            // history with one entry so the very first back press fires popstate
            // instead of closing the WebView.
            _ = document::eval(r#"
                window.__recallo_back_count = 0;
                window.addEventListener('popstate', function(_e) {
                    window.__recallo_back_count++;
                    // Re-push so there is always a history entry to pop
                    history.pushState(null, '');
                });
                // Seed initial history entry
                history.pushState(null, '');
            "#);

            let mut seen: i64 = 0;

            loop {
                // Short sleep to avoid busy-looping — 100 ms is responsive enough
                #[cfg(not(target_arch = "wasm32"))]
                async_std::task::sleep(std::time::Duration::from_millis(100)).await;
                #[cfg(target_arch = "wasm32")]
                gloo_timers::future::sleep(std::time::Duration::from_millis(100)).await;

                // Read the JS counter
                let mut eval = document::eval(r#"
                    dioxus.send(window.__recallo_back_count || 0);
                "#);
                if let Ok(count) = eval.recv::<i64>().await {
                    while seen < count {
                        seen += 1;
                        // Navigate back in app history
                        if let Some(prev) = history.write().pop() {
                            current_screen.set(prev);
                        }
                        // If history is empty we're at Home — let Android
                        // handle the next back press (it will close the app).
                    }
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
