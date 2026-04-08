use crate::state::Screen;
use dioxus::prelude::*;

// ── Navigation ──────────────────────────────────────────────────────────────

/// Push current screen onto history stack and navigate to a new screen.
pub fn navigate(
    current_screen: &mut Signal<Screen>,
    history: &mut Signal<Vec<Screen>>,
    target: Screen,
) {
    history.write().push(current_screen.read().clone());
    current_screen.set(target);
}

/// Pop the previous screen from history, or fall back to Home.
pub fn go_back(current_screen: &mut Signal<Screen>, history: &mut Signal<Vec<Screen>>) {
    if let Some(prev) = history.write().pop() {
        current_screen.set(prev);
    } else {
        current_screen.set(Screen::Home);
    }
}

// ── Time formatting ─────────────────────────────────────────────────────────

/// Format seconds as "MM:SS.t" (with minutes) or "S.ts" (without).
pub fn format_time(secs: f64) -> String {
    let mins = (secs / 60.0) as u32;
    let sec_part = (secs % 60.0) as u32;
    let tenths = ((secs % 1.0) * 10.0) as u32;
    if mins > 0 {
        format!("{mins:02}:{sec_part:02}.{tenths}")
    } else {
        format!("{sec_part}.{tenths}s")
    }
}

// ── Cross-platform time ─────────────────────────────────────────────────────

/// Current wall-clock time in milliseconds (monotonic enough for a quiz timer).
pub fn time_now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as f64)
            .unwrap_or(0.0)
    }
}

/// Cross-platform async sleep.
pub async fn async_sleep_ms(ms: u64) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms as u32).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        async_std::task::sleep(std::time::Duration::from_millis(ms)).await;
    }
}

// ── Answer checking ─────────────────────────────────────────────────────────

/// Check a typed answer against the correct answer (case-insensitive substring match).
pub fn check_typed_answer(user_input: &str, correct: &str) -> bool {
    let user = user_input.trim().to_lowercase();
    let correct = correct.to_lowercase();
    !user.is_empty() && (correct.contains(user.as_str()) || user.contains(correct.as_str()))
}
