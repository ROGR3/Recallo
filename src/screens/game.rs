use dioxus::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::data::{Word, WORDS, MathEntry, MATH_ENTRIES};
use crate::state::{GameConfig, GameResult, Progress, Screen, Subject, PENALTY_SECONDS};

const CHOICES: usize = 4;
const TYPING_PROBABILITY: f64 = 0.10;

#[derive(Debug, Clone, PartialEq, Eq)]
enum QuestionDirection {
    /// Korean→English or Name→Statement
    Forward,
    /// English→Korean (Korean only)
    Reverse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InputMode {
    MultipleChoice,
    Typing,
}

/// A unified question representation for both Korean and Math.
#[derive(Debug, Clone, PartialEq)]
struct Question {
    /// The unique key used for "I know this" tracking
    known_key: String,
    /// The text shown as the prompt
    prompt: String,
    /// The correct answer text
    correct_answer: String,
    /// Label above the prompt (e.g. "What does this mean?" or "Definice")
    prompt_label: String,
    /// Direction of the question
    direction: QuestionDirection,
    input_mode: InputMode,
    /// All choices (for MC), including the correct one
    choices: Vec<String>,
    /// Index of the correct choice in `choices`
    correct_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum AnswerState {
    Waiting,
    Correct,
    Wrong { correct_answer: String },
}

fn build_korean_questions(config: &GameConfig, progress: &Progress) -> Vec<Question> {
    let mut rng = rand::thread_rng();
    let game_size = config.mode.word_count();

    let cat_key = config.category.as_deref();
    let pool: Vec<&'static Word> = WORDS.iter().filter(|w| {
        let cat_ok = cat_key.is_none_or(|k| w.category.display_name().to_lowercase() == k);
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
        let direction = if rng.gen_bool(0.5) {
            QuestionDirection::Forward
        } else {
            QuestionDirection::Reverse
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

        let mut choice_words: Vec<&'static Word> = vec![word];
        choice_words.extend(distractors);
        choice_words.shuffle(&mut rng);

        let correct_index = choice_words.iter().position(|w| w.korean == word.korean).unwrap();

        let (prompt, correct_answer, prompt_label, choices) = match direction {
            QuestionDirection::Forward => (
                word.korean.to_string(),
                word.english.to_string(),
                "What does this mean?".to_string(),
                choice_words.iter().map(|w| w.english.to_string()).collect(),
            ),
            QuestionDirection::Reverse => (
                word.english.to_string(),
                word.korean.to_string(),
                "How do you say this in Korean?".to_string(),
                choice_words.iter().map(|w| w.korean.to_string()).collect(),
            ),
        };

        Question {
            known_key: word.korean.to_string(),
            prompt,
            correct_answer,
            prompt_label,
            direction,
            input_mode,
            choices,
            correct_index,
        }
    }).collect()
}

fn build_math_questions(config: &GameConfig, progress: &Progress) -> Vec<Question> {
    let mut rng = rand::thread_rng();
    let game_size = config.mode.word_count();

    let subject_filter = match config.subject {
        Subject::MathAnalysis => crate::data::MathSubject::Analysis,
        Subject::MathDataScience => crate::data::MathSubject::DataScience,
        _ => unreachable!(),
    };

    let cat_key = config.category.as_deref();
    let pool: Vec<&'static MathEntry> = MATH_ENTRIES.iter().filter(|e| {
        if e.subject != subject_filter { return false; }
        let topic_ok = cat_key.is_none_or(|k| e.topic.key() == k);
        if !topic_ok { return false; }
        let is_known = progress.is_known(e.name);
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

    // Distractor pool: all entries from the same subject
    let distractor_pool: Vec<&'static MathEntry> = MATH_ENTRIES.iter()
        .filter(|e| e.subject == subject_filter)
        .collect();

    let count = game_size.min(pool.len());
    let selected: Vec<&'static MathEntry> = pool
        .choose_multiple(&mut rng, count)
        .cloned()
        .collect();

    selected.into_iter().map(|entry| {
        let input_mode = if rng.gen_bool(TYPING_PROBABILITY) {
            InputMode::Typing
        } else {
            InputMode::MultipleChoice
        };

        let distractors: Vec<&'static MathEntry> = distractor_pool.iter()
            .filter(|e| e.name != entry.name)
            .cloned()
            .collect::<Vec<_>>()
            .choose_multiple(&mut rng, CHOICES - 1)
            .cloned()
            .collect();

        let mut choice_entries: Vec<&'static MathEntry> = vec![entry];
        choice_entries.extend(distractors);
        choice_entries.shuffle(&mut rng);

        let correct_index = choice_entries.iter().position(|e| e.name == entry.name).unwrap();

        let prompt_label = format!("{} — {}", entry.entry_type.display_name(), entry.topic.display_name());

        Question {
            known_key: entry.name.to_string(),
            prompt: entry.name.to_string(),
            correct_answer: entry.statement.to_string(),
            prompt_label,
            direction: QuestionDirection::Forward,
            input_mode,
            choices: choice_entries.iter().map(|e| e.statement.to_string()).collect(),
            correct_index,
        }
    }).collect()
}

fn build_questions(config: &GameConfig, progress: &Progress) -> Vec<Question> {
    match config.subject {
        Subject::Korean => build_korean_questions(config, progress),
        Subject::MathAnalysis | Subject::MathDataScience => build_math_questions(config, progress),
    }
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

#[allow(clippy::too_many_arguments)]
fn go_to_results(
    config: &GameConfig,
    final_secs: f64,
    penalty_secs: f64,
    final_score: usize,
    mistakes: usize,
    q_count: usize,
    progress: &mut Signal<Progress>,
    current_screen: &mut Signal<Screen>,
) {
    let total_time = final_secs + penalty_secs;
    let key = config.best_time_key();
    let prev_best = progress.read().get_best_time(&key);
    let is_new_best = progress.write().maybe_update_best_time(&key, total_time);
    crate::state::save_progress(&progress.read());
    let result = GameResult {
        config: config.clone(),
        score: final_score,
        total: q_count,
        time_seconds: total_time,
        penalty_seconds: penalty_secs,
        mistakes,
        is_new_best,
        previous_best: prev_best,
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
    let mut mistakes: Signal<usize> = use_signal(|| 0);
    let mut penalty_total: Signal<f64> = use_signal(|| 0.0);
    let mut answer_state: Signal<AnswerState> = use_signal(|| AnswerState::Waiting);
    let mut typing_input: Signal<String> = use_signal(String::new);
    let mut marked_known: Signal<bool> = use_signal(|| false);
    // Show penalty popup briefly
    let mut show_penalty: Signal<bool> = use_signal(|| false);

    let mut start_ms: Signal<f64> = use_signal(get_time_ms);
    let mut accumulated_secs: Signal<f64> = use_signal(|| 0.0);
    let mut paused: Signal<bool> = use_signal(|| false);

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
        let penalty = *penalty_total.peek();
        if *paused.peek() {
            acc + penalty
        } else {
            let now = get_time_ms();
            let start = *start_ms.peek();
            acc + (now - start) / 1000.0 + penalty
        }
    };

    let q_count = questions.read().len();

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

    let question = questions.read()[idx].clone();
    let known_key = question.known_key.clone();
    let known_key_for_mark = question.known_key.clone();

    let is_typing = question.input_mode == InputMode::Typing;
    let is_answered = *answer_state.read() != AnswerState::Waiting;
    let is_last = idx + 1 >= q_count;

    let is_math = config.subject.is_math();

    let penalty_secs = PENALTY_SECONDS as u32;

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

            // Score row
            div { class: "score-row",
                span { class: "score-badge", "✓ {sc}" }
            }

            // Penalty popup overlay
            if *show_penalty.read() {
                div { class: "penalty-popup",
                    span { class: "penalty-popup-text", "+{penalty_secs}s" }
                }
            }

            // Question card
            div { class: "question-card",
                p { class: "question-label", "{question.prompt_label}" }
                div { class: if is_math { "question-prompt question-prompt--math" } else { "question-prompt" },
                    span { class: "prompt-text", "{question.prompt}" }
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
                                let correct_answer = question.correct_answer.clone();
                                move |e: KeyboardEvent| {
                                    if e.key() == Key::Enter && !is_answered {
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
                                            // Add penalty
                                            penalty_total += PENALTY_SECONDS;
                                            mistakes += 1;
                                            show_penalty.set(true);
                                            answer_state.set(AnswerState::Wrong {
                                                correct_answer: correct_answer.clone()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        if !is_answered {
                            button {
                                class: "submit-btn",
                                onclick: {
                                    let correct_answer = question.correct_answer.clone();
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
                                            penalty_total += PENALTY_SECONDS;
                                            mistakes += 1;
                                            show_penalty.set(true);
                                            answer_state.set(AnswerState::Wrong {
                                                correct_answer: correct_answer.clone()
                                            });
                                        }
                                    }
                                },
                                "Submit"
                            }
                        }
                    }
                } else {
                    // Multiple choice
                    div { class: if is_math { "choices-grid choices-grid--math" } else { "choices-grid" },
                        for i in 0..question.choices.len() {
                            {
                                let choice_text = question.choices[i].clone();
                                let is_correct_choice = i == question.correct_index;
                                let ans_state = answer_state.read().clone();
                                let correct_answer_mc = question.correct_answer.clone();
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
                                let btn_class_math = if is_math {
                                    format!("{btn_class} choice-btn--math")
                                } else {
                                    btn_class.to_string()
                                };
                                rsx! {
                                    button {
                                        key: "{i}",
                                        class: "{btn_class_math}",
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
                                                    penalty_total += PENALTY_SECONDS;
                                                    mistakes += 1;
                                                    show_penalty.set(true);
                                                    answer_state.set(AnswerState::Wrong {
                                                        correct_answer: correct_answer_mc.clone()
                                                    });
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

                // Feedback + post-answer actions
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
                                    span { class: "feedback-wrong-header",
                                        span { "✗ Wrong" }
                                        span { class: "feedback-penalty", "+{penalty_secs}s" }
                                    }
                                    span { class: "feedback-correct", "{ca}" }
                                }
                            },
                            AnswerState::Waiting => rsx! { div {} }
                        }

                        div { class: "post-answer-actions",
                            if !progress.read().is_known(&known_key) && !*marked_known.read() {
                                button {
                                    class: "know-btn",
                                    onclick: move |_| {
                                        progress.write().mark_known(&known_key_for_mark);
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
                                    if is_last {
                                        let final_secs = *accumulated_secs.read();
                                        let penalty = *penalty_total.read();
                                        let final_score = *score.read();
                                        let mistake_count = *mistakes.read();
                                        go_to_results(&config_for_results, final_secs, penalty, final_score, mistake_count, q_count, &mut progress, &mut current_screen);
                                    } else {
                                        question_index += 1;
                                        answer_state.set(AnswerState::Waiting);
                                        typing_input.set(String::new());
                                        marked_known.set(false);
                                        show_penalty.set(false);
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
