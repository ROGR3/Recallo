use dioxus::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::data::{Word, WORDS};
use crate::state::{GameConfig, GameResult, Progress, Screen};

const CHOICES: usize = 4;
const TYPING_PROBABILITY: f64 = 0.10;

#[derive(Debug, Clone, PartialEq, Eq)]
enum QuestionMode {
    KoreanToEnglish,
    EnglishToKorean,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InputMode {
    MultipleChoice,
    Typing,
}

#[derive(Debug, Clone, PartialEq)]
struct Question {
    word_korean: &'static str,
    word_english: &'static str,
    mode: QuestionMode,
    input_mode: InputMode,
    choice_koreans: Vec<&'static str>,
    choice_englishes: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
enum AnswerState {
    Waiting,
    Correct,
    Wrong { correct_answer: String },
}

fn build_questions(config: &GameConfig, progress: &Progress) -> Vec<Question> {
    let mut rng = rand::thread_rng();
    let game_size = config.mode.word_count();

    let pool: Vec<&'static Word> = WORDS.iter().filter(|w| {
        let cat_ok = config.category.as_ref().is_none_or(|c| &w.category == c);
        if !cat_ok { return false; }
        let is_known = progress.is_known(w.korean);
        match (config.include_unknown, config.include_known) {
            (true, true) => true,
            (true, false) => !is_known,
            (false, true) => is_known,
            (false, false) => true,
        }
    }).collect();

    if pool.is_empty() {
        return vec![];
    }

    let count = game_size.min(pool.len());
    let selected: Vec<&'static Word> = pool
        .choose_multiple(&mut rng, count)
        .cloned()
        .collect();

    selected.into_iter().map(|word| {
        let mode = if rng.gen_bool(0.5) {
            QuestionMode::KoreanToEnglish
        } else {
            QuestionMode::EnglishToKorean
        };

        let input_mode = if rng.gen_bool(TYPING_PROBABILITY) {
            InputMode::Typing
        } else {
            InputMode::MultipleChoice
        };

        let distractors: Vec<&'static Word> = WORDS.iter()
            .filter(|w| w.korean != word.korean)
            .collect::<Vec<_>>()
            .choose_multiple(&mut rng, CHOICES - 1)
            .cloned()
            .collect();

        let mut choices: Vec<&'static Word> = vec![word];
        choices.extend(distractors);
        choices.shuffle(&mut rng);

        Question {
            word_korean: word.korean,
            word_english: word.english,
            mode,
            input_mode,
            choice_koreans: choices.iter().map(|w| w.korean).collect(),
            choice_englishes: choices.iter().map(|w| w.english).collect(),
        }
    }).collect()
}

fn get_time_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    { js_sys::Date::now() }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as f64)
            .unwrap_or(0.0)
    }
}

/// Navigate to results screen
fn go_to_results(
    config: &GameConfig,
    final_secs: f64,
    final_score: usize,
    q_count: usize,
    failed_perfect: bool,
    progress: &mut Signal<Progress>,
    current_screen: &mut Signal<Screen>,
) {
    let key = config.best_time_key();
    let prev_best = progress.read().get_best_time(&key);
    // Only record best time if the game was completed successfully (not failed perfect)
    let is_new_best = if !failed_perfect {
        progress.write().maybe_update_best_time(&key, final_secs)
    } else {
        false
    };
    crate::state::save_progress(&progress.read());
    let result = GameResult {
        config: config.clone(),
        score: final_score,
        total: q_count,
        time_seconds: final_secs,
        is_new_best,
        previous_best: prev_best,
        failed_perfect,
    };
    current_screen.set(Screen::Results { result });
}

#[component]
pub fn GameScreen(
    config: GameConfig,
    mut current_screen: Signal<Screen>,
    mut progress: Signal<Progress>,
) -> Element {
    let questions: Signal<Vec<Question>> = use_signal({
        let config = config.clone();
        let progress_snap = progress.read().clone();
        move || build_questions(&config, &progress_snap)
    });

    let mut question_index: Signal<usize> = use_signal(|| 0);
    let mut score: Signal<usize> = use_signal(|| 0);
    let mut answer_state: Signal<AnswerState> = use_signal(|| AnswerState::Waiting);
    let mut typing_input: Signal<String> = use_signal(String::new);
    let mut marked_known: Signal<bool> = use_signal(|| false);

    let mut start_ms: Signal<f64> = use_signal(get_time_ms);
    let mut accumulated_secs: Signal<f64> = use_signal(|| 0.0);
    let mut paused: Signal<bool> = use_signal(|| false);

    // Tick every 100ms for timer display
    use_coroutine(move |_rx: UnboundedReceiver<()>| async move {
        loop {
            async_std_time_sleep(100).await;
            if !*paused.read() {
                let _ = *accumulated_secs.read();
            }
        }
    });

    let display_secs: f64 = {
        let acc = *accumulated_secs.peek();
        if *paused.peek() {
            acc
        } else {
            let now = get_time_ms();
            let start = *start_ms.peek();
            acc + (now - start) / 1000.0
        }
    };

    let q_count = questions.read().len();
    let is_perfect_mode = config.mode.is_no_mistakes();

    if q_count == 0 {
        return rsx! {
            div { class: "screen",
                div { class: "screen-header",
                    button {
                        class: "back-btn",
                        onclick: move |_| current_screen.set(Screen::Home),
                        "←"
                    }
                    h1 { class: "screen-title", "No words" }
                }
                div { class: "empty-state",
                    p { "No words available for this selection." }
                    p { "Try changing the category or filters." }
                    button {
                        class: "start-btn",
                        onclick: move |_| current_screen.set(Screen::Home),
                        "Back to Home"
                    }
                }
            }
        };
    }

    let idx = *question_index.read();
    let sc = *score.read();
    let mins = (display_secs / 60.0) as u32;
    let sec_part = (display_secs % 60.0) as u32;
    let progress_pct = if q_count > 0 { idx * 100 / q_count } else { 0 };

    let config_for_results = config.clone();
    let config_for_fail = config.clone();

    let question = questions.read()[idx].clone();
    let word_korean = question.word_korean;
    let word_english = question.word_english;

    let prompt_text: &'static str = match question.mode {
        QuestionMode::KoreanToEnglish => word_korean,
        QuestionMode::EnglishToKorean => word_english,
    };

    let correct_answer: String = match question.mode {
        QuestionMode::KoreanToEnglish => word_english.to_string(),
        QuestionMode::EnglishToKorean => word_korean.to_string(),
    };

    let prompt_label: &'static str = match question.mode {
        QuestionMode::KoreanToEnglish => "What does this mean?",
        QuestionMode::EnglishToKorean => "How do you say this in Korean?",
    };

    let is_typing = question.input_mode == InputMode::Typing;
    let is_answered = *answer_state.read() != AnswerState::Waiting;
    let is_last = idx + 1 >= q_count;

    let choice_koreans = question.choice_koreans.clone();
    let choice_englishes = question.choice_englishes.clone();

    let mode_label = config.mode.display_name();

    rsx! {
        div { class: "screen game-screen",
            // Header
            div { class: "game-header",
                button {
                    class: "back-btn",
                    onclick: move |_| current_screen.set(Screen::Home),
                    "✕"
                }
                div { class: "progress-info",
                    span { class: "progress-text", "{idx + 1} / {q_count}" }
                }
                div { class: "timer-display",
                    span { class: "timer-text", "{mins:02}:{sec_part:02}" }
                }
            }

            // Progress bar
            div { class: "progress-bar-container",
                div { class: "progress-bar-fill", style: "width: {progress_pct}%" }
            }

            // Score + mode row
            div { class: "score-row",
                span { class: "score-badge", "✓ {sc}" }
                if is_perfect_mode {
                    span { class: "mode-badge mode-badge--perfect", "{config.mode.emoji()} {mode_label}" }
                } else {
                    span { class: "mode-badge",
                        if is_typing { "✏️ Type it" } else { "👆 Pick it" }
                    }
                }
            }

            // Question card
            div { class: "question-card",
                p { class: "question-label", "{prompt_label}" }
                div { class: "question-prompt",
                    span { class: "prompt-text", "{prompt_text}" }
                }
            }

            // Answer area
            div { class: "answer-area",
                if is_typing {
                    div { class: "typing-section",
                        input {
                            class: "type-input",
                            r#type: "text",
                            placeholder: "Type your answer...",
                            value: "{typing_input}",
                            disabled: is_answered,
                            oninput: move |e| typing_input.set(e.value()),
                            onkeydown: {
                                let correct_answer = correct_answer.clone();
                                let config_fail = config_for_fail.clone();
                                move |e: KeyboardEvent| {
                                    if e.key() == Key::Enter && !is_answered {
                                        let user = typing_input.read().trim().to_lowercase();
                                        let correct = correct_answer.to_lowercase();
                                        let is_correct = !user.is_empty() && (
                                            correct.contains(user.as_str())
                                            || user.contains(correct.as_str())
                                        );
                                        // Pause timer
                                        if !*paused.read() {
                                            let now = get_time_ms();
                                            let acc_val = *accumulated_secs.peek();
                                            let start_val = *start_ms.peek();
                                            accumulated_secs.set(acc_val + (now - start_val) / 1000.0);
                                            paused.set(true);
                                        }
                                        if is_correct {
                                            score += 1;
                                            answer_state.set(AnswerState::Correct);
                                        } else {
                                            answer_state.set(AnswerState::Wrong {
                                                correct_answer: correct_answer.clone()
                                            });
                                            // Perfect mode: end game immediately on wrong answer
                                            if is_perfect_mode {
                                                let final_secs = *accumulated_secs.read();
                                                let final_score = *score.read();
                                                go_to_results(&config_fail, final_secs, final_score, q_count, true, &mut progress, &mut current_screen);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !is_answered {
                            button {
                                class: "submit-btn",
                                onclick: {
                                    let correct_answer = correct_answer.clone();
                                    let config_fail = config_for_fail.clone();
                                    move |_| {
                                        let user = typing_input.read().trim().to_lowercase();
                                        let correct = correct_answer.to_lowercase();
                                        let is_correct = !user.is_empty() && (
                                            correct.contains(user.as_str())
                                            || user.contains(correct.as_str())
                                        );
                                        if !*paused.read() {
                                            let now = get_time_ms();
                                            let acc_val = *accumulated_secs.peek();
                                            let start_val = *start_ms.peek();
                                            accumulated_secs.set(acc_val + (now - start_val) / 1000.0);
                                            paused.set(true);
                                        }
                                        if is_correct {
                                            score += 1;
                                            answer_state.set(AnswerState::Correct);
                                        } else {
                                            answer_state.set(AnswerState::Wrong {
                                                correct_answer: correct_answer.clone()
                                            });
                                            if is_perfect_mode {
                                                let final_secs = *accumulated_secs.read();
                                                let final_score = *score.read();
                                                go_to_results(&config_fail, final_secs, final_score, q_count, true, &mut progress, &mut current_screen);
                                            }
                                        }
                                    }
                                },
                                "Submit"
                            }
                        }
                    }
                } else {
                    // Multiple choice
                    div { class: "choices-grid",
                        for i in 0..choice_koreans.len() {
                            {
                                let c_korean = choice_koreans[i];
                                let c_english = choice_englishes[i];
                                let choice_text: &'static str = match question.mode {
                                    QuestionMode::KoreanToEnglish => c_english,
                                    QuestionMode::EnglishToKorean => c_korean,
                                };
                                let is_correct_choice = c_korean == word_korean;
                                let ans_state = answer_state.read().clone();
                                let correct_answer_mc = correct_answer.clone();
                                let config_fail_mc = config_for_fail.clone();
                                let btn_class = match &ans_state {
                                    AnswerState::Waiting => "choice-btn",
                                    AnswerState::Correct => {
                                        if is_correct_choice { "choice-btn choice-btn--correct" }
                                        else { "choice-btn choice-btn--dimmed" }
                                    }
                                    AnswerState::Wrong { .. } => {
                                        if is_correct_choice { "choice-btn choice-btn--reveal" }
                                        else { "choice-btn choice-btn--dimmed" }
                                    }
                                };
                                rsx! {
                                    button {
                                        key: "{c_korean}",
                                        class: "{btn_class}",
                                        disabled: is_answered,
                                        onclick: move |_| {
                                            if !is_answered {
                                                if !*paused.read() {
                                                    let now = get_time_ms();
                                                    let acc_val = *accumulated_secs.peek();
                                                    let start_val = *start_ms.peek();
                                                    accumulated_secs.set(acc_val + (now - start_val) / 1000.0);
                                                    paused.set(true);
                                                }
                                                if is_correct_choice {
                                                    score += 1;
                                                    answer_state.set(AnswerState::Correct);
                                                } else {
                                                    answer_state.set(AnswerState::Wrong {
                                                        correct_answer: correct_answer_mc.clone()
                                                    });
                                                    if is_perfect_mode {
                                                        let final_secs = *accumulated_secs.read();
                                                        let final_score = *score.read();
                                                        go_to_results(&config_fail_mc, final_secs, final_score, q_count, true, &mut progress, &mut current_screen);
                                                    }
                                                }
                                            }
                                        },
                                        "{choice_text}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Feedback + post-answer actions (only in normal mode, or on correct in perfect)
                if is_answered {
                    div { class: "feedback-section",
                        match answer_state.read().clone() {
                            AnswerState::Correct => rsx! {
                                div { class: "feedback feedback--correct",
                                    span { "✓ Correct!" }
                                }
                            },
                            AnswerState::Wrong { correct_answer: ca } => rsx! {
                                div { class: "feedback feedback--wrong",
                                    span { "✗ Wrong" }
                                    span { class: "feedback-correct", "Answer: {ca}" }
                                }
                            },
                            AnswerState::Waiting => rsx! { div {} }
                        }

                        div { class: "post-answer-actions",
                            if !progress.read().is_known(word_korean) && !*marked_known.read() {
                                button {
                                    class: "know-btn",
                                    onclick: move |_| {
                                        progress.write().mark_known(word_korean);
                                        crate::state::save_progress(&progress.read());
                                        marked_known.set(true);
                                    },
                                    "✓ I know this word"
                                }
                            } else if *marked_known.read() {
                                div { class: "known-badge", "Marked as known ✓" }
                            }

                            button {
                                class: "next-btn",
                                onclick: move |_| {
                                    let final_secs = *accumulated_secs.read();
                                    let final_score = *score.read();
                                    if is_last {
                                        go_to_results(&config_for_results, final_secs, final_score, q_count, false, &mut progress, &mut current_screen);
                                    } else {
                                        question_index += 1;
                                        answer_state.set(AnswerState::Waiting);
                                        typing_input.set(String::new());
                                        marked_known.set(false);
                                        start_ms.set(get_time_ms());
                                        paused.set(false);
                                    }
                                },
                                if is_last { "Finish →" } else { "Next →" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Cross-platform async sleep for the timer coroutine
async fn async_std_time_sleep(ms: u64) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms as u32).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        async_std::task::sleep(std::time::Duration::from_millis(ms)).await;
    }
}
