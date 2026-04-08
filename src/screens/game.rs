use crate::data::{word_matches_category, MathEntry, Word, MATH_ENTRIES, WORDS};
use crate::state::{GameConfig, GameResult, Progress, Screen, Subject, PENALTY_SECONDS};
use crate::util::{async_sleep_ms, check_typed_answer, time_now_ms};
use dioxus::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

const CHOICES: usize = 4;
const TYPING_PROBABILITY: f64 = 0.10;
const FORWARD_PROBABILITY: f64 = 0.50;
const TIMER_POLL_MS: u64 = 100;

// ── Question model ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
enum QuestionDirection {
    /// Korean->English or Name->Statement
    Forward,
    /// English->Korean (Korean only)
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
    /// Key used for "I know this" tracking
    known_key: String,
    /// Text shown as the prompt
    prompt: String,
    /// The correct answer text
    correct_answer: String,
    /// Label above the prompt (e.g. "What does this mean?")
    prompt_label: String,
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

// ── Question building ───────────────────────────────────────────────────────

fn pick_input_mode(rng: &mut impl Rng) -> InputMode {
    if rng.gen_bool(TYPING_PROBABILITY) {
        InputMode::Typing
    } else {
        InputMode::MultipleChoice
    }
}

fn build_korean_questions(config: &GameConfig, progress: &Progress) -> Vec<Question> {
    let mut rng = rand::thread_rng();

    let cat_key = config.category.as_deref();
    let pool: Vec<&'static Word> = WORDS
        .iter()
        .filter(|w| {
            word_matches_category(w, cat_key)
                && config.should_include(progress.is_known(w.korean))
        })
        .collect();

    if pool.is_empty() {
        return vec![];
    }

    let count = config.mode.word_count().min(pool.len());
    let selected: Vec<&'static Word> = pool.choose_multiple(&mut rng, count).cloned().collect();

    selected
        .into_iter()
        .map(|word| {
            let is_forward = rng.gen_bool(FORWARD_PROBABILITY);
            let input_mode = pick_input_mode(&mut rng);

            // Build distractor choices from full word pool
            let distractors: Vec<&'static Word> = WORDS
                .iter()
                .filter(|w| w.korean != word.korean)
                .collect::<Vec<_>>()
                .choose_multiple(&mut rng, CHOICES - 1)
                .cloned()
                .collect();

            let mut choice_words: Vec<&'static Word> = vec![word];
            choice_words.extend(distractors);
            choice_words.shuffle(&mut rng);
            let correct_index = choice_words
                .iter()
                .position(|w| w.korean == word.korean)
                .unwrap();

            let (prompt, correct_answer, prompt_label, choices) = if is_forward {
                (
                    word.korean.to_string(),
                    word.english.to_string(),
                    "Co to znamen\u{00e1}?".to_string(),
                    choice_words.iter().map(|w| w.english.to_string()).collect(),
                )
            } else {
                (
                    word.english.to_string(),
                    word.korean.to_string(),
                    "Jak se to \u{0159}ekne korejsky?".to_string(),
                    choice_words.iter().map(|w| w.korean.to_string()).collect(),
                )
            };

            Question {
                known_key: word.korean.to_string(),
                prompt,
                correct_answer,
                prompt_label,
                input_mode,
                choices,
                correct_index,
            }
        })
        .collect()
}

fn build_math_questions(config: &GameConfig, progress: &Progress) -> Vec<Question> {
    let mut rng = rand::thread_rng();
    let subject_filter = config.subject.math_subject().expect("must be math subject");

    let cat_key = config.category.as_deref();
    let type_filter = config.entry_type_filter;
    let pool: Vec<&'static MathEntry> = MATH_ENTRIES
        .iter()
        .filter(|e| {
            e.subject == subject_filter
                && cat_key.is_none_or(|k| e.topic.key() == k)
                && type_filter.matches(e.entry_type)
                && config.should_include(progress.is_known(e.name))
        })
        .collect();

    if pool.is_empty() {
        return vec![];
    }

    // Distractors from the same math subject and entry type
    let distractor_pool: Vec<&'static MathEntry> = MATH_ENTRIES
        .iter()
        .filter(|e| e.subject == subject_filter && type_filter.matches(e.entry_type))
        .collect();

    let count = config.mode.word_count().min(pool.len());
    let selected: Vec<&'static MathEntry> = pool.choose_multiple(&mut rng, count).cloned().collect();

    selected
        .into_iter()
        .map(|entry| {
            let input_mode = pick_input_mode(&mut rng);

            let distractors: Vec<&'static MathEntry> = distractor_pool
                .iter()
                .filter(|e| e.name != entry.name)
                .cloned()
                .collect::<Vec<_>>()
                .choose_multiple(&mut rng, CHOICES - 1)
                .cloned()
                .collect();

            let mut choice_entries: Vec<&'static MathEntry> = vec![entry];
            choice_entries.extend(distractors);
            choice_entries.shuffle(&mut rng);
            let correct_index = choice_entries
                .iter()
                .position(|e| e.name == entry.name)
                .unwrap();

            let prompt_label = format!(
                "{} \u{2014} {}",
                entry.entry_type.display_name(),
                entry.topic.display_name()
            );

            Question {
                known_key: entry.name.to_string(),
                prompt: entry.name.to_string(),
                correct_answer: entry.statement.to_string(),
                prompt_label,
                input_mode,
                choices: choice_entries
                    .iter()
                    .map(|e| e.statement.to_string())
                    .collect(),
                correct_index,
            }
        })
        .collect()
}

fn build_questions(config: &GameConfig, progress: &Progress) -> Vec<Question> {
    match config.subject {
        Subject::Korean => build_korean_questions(config, progress),
        Subject::MathAnalysis | Subject::MathDataScience => build_math_questions(config, progress),
    }
}

// ── Timer helpers ───────────────────────────────────────────────────────────

/// Pause the running timer and accumulate elapsed time.
fn pause_timer(
    mut paused: Signal<bool>,
    mut accumulated_secs: Signal<f64>,
    start_ms: Signal<f64>,
) {
    if !*paused.read() {
        let elapsed = (time_now_ms() - *start_ms.peek()) / 1000.0;
        *accumulated_secs.write() += elapsed;
        paused.set(true);
    }
}

/// Resume the timer from now.
fn resume_timer(paused: &mut Signal<bool>, start_ms: &mut Signal<f64>) {
    start_ms.set(time_now_ms());
    paused.set(false);
}

/// Compute the current display time (accumulated + live elapsed + penalty).
fn display_time(
    accumulated_secs: &Signal<f64>,
    penalty_total: &Signal<f64>,
    paused: &Signal<bool>,
    start_ms: &Signal<f64>,
) -> f64 {
    let acc = *accumulated_secs.peek();
    let penalty = *penalty_total.peek();
    if *paused.peek() {
        acc + penalty
    } else {
        let live_elapsed = (time_now_ms() - *start_ms.peek()) / 1000.0;
        acc + live_elapsed + penalty
    }
}

// ── Answer handling ─────────────────────────────────────────────────────────

/// Process a submitted answer: pause timer, update score/penalty/state.
fn submit_answer(
    is_correct: bool,
    correct_answer: &str,
    paused: Signal<bool>,
    accumulated_secs: Signal<f64>,
    start_ms: Signal<f64>,
    mut score: Signal<usize>,
    mut penalty_total: Signal<f64>,
    mut mistakes: Signal<usize>,
    mut show_penalty: Signal<bool>,
    mut answer_state: Signal<AnswerState>,
) {
    pause_timer(paused, accumulated_secs, start_ms);

    if is_correct {
        *score.write() += 1;
        answer_state.set(AnswerState::Correct);
    } else {
        *penalty_total.write() += PENALTY_SECONDS;
        *mistakes.write() += 1;
        show_penalty.set(true);
        answer_state.set(AnswerState::Wrong {
            correct_answer: correct_answer.to_string(),
        });
    }
}

// ── Results transition ──────────────────────────────────────────────────────

fn go_to_results(
    config: &GameConfig,
    accumulated_secs: f64,
    penalty_secs: f64,
    final_score: usize,
    mistakes: usize,
    total: usize,
    progress: &mut Signal<Progress>,
    current_screen: &mut Signal<Screen>,
    history: &mut Signal<Vec<Screen>>,
) {
    let total_time = accumulated_secs + penalty_secs;
    let key = config.best_time_key();
    let prev_best = progress.read().get_best_time(&key);
    let is_new_best = progress.write().maybe_update_best_time(&key, total_time);
    crate::state::save_progress(&progress.read());

    let result = GameResult {
        config: config.clone(),
        score: final_score,
        total,
        time_seconds: total_time,
        penalty_seconds: penalty_secs,
        mistakes,
        is_new_best,
        previous_best: prev_best,
    };
    history.write().clear();
    current_screen.set(Screen::Results { result });
}

// ── Game screen component ───────────────────────────────────────────────────

#[component]
pub fn GameScreen(
    config: GameConfig,
    mut current_screen: Signal<Screen>,
    mut history: Signal<Vec<Screen>>,
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
    let mut show_penalty: Signal<bool> = use_signal(|| false);

    let mut start_ms: Signal<f64> = use_signal(time_now_ms);
    let mut accumulated_secs: Signal<f64> = use_signal(|| 0.0);
    let mut paused: Signal<bool> = use_signal(|| false);

    // Timer tick — forces re-render while running
    use_coroutine(move |_rx: UnboundedReceiver<()>| async move {
        loop {
            async_sleep_ms(TIMER_POLL_MS).await;
            if !*paused.read() {
                let _ = *accumulated_secs.read();
            }
        }
    });

    let secs = display_time(&accumulated_secs, &penalty_total, &paused, &start_ms);
    let q_count = questions.read().len();

    if q_count == 0 {
        return rsx! {
            div { class: "screen",
                div { class: "screen-header",
                    button {
                        class: "back-btn",
                        onclick: move |_| { history.write().clear(); current_screen.set(Screen::Home); },
                        "\u{2190}"
                    }
                    h1 { class: "screen-title", "\u{017d}\u{00e1}dn\u{00e9} ot\u{00e1}zky" }
                }
                div { class: "empty-state",
                    p { "Pro tento v\u{00fd}b\u{011b}r nejsou dostupn\u{00e9} \u{017e}\u{00e1}dn\u{00e9} ot\u{00e1}zky." }
                    p { "Zkus zm\u{011b}nit kategorii nebo filtry." }
                    button {
                        class: "start-btn",
                        onclick: move |_| { history.write().clear(); current_screen.set(Screen::Home); },
                        "Zp\u{011b}t na hlavn\u{00ed}"
                    }
                }
            }
        };
    }

    let idx = *question_index.read();
    let sc = *score.read();
    let mins = (secs / 60.0) as u32;
    let sec_part = (secs % 60.0) as u32;
    let progress_pct = idx * 100 / q_count;
    let is_last = idx + 1 >= q_count;
    let is_math = config.subject.is_math();
    let penalty_display = PENALTY_SECONDS as u32;

    let question = questions.read()[idx].clone();
    let known_key = question.known_key.clone();
    let known_key_mark = question.known_key.clone();
    let is_typing = question.input_mode == InputMode::Typing;
    let is_answered = *answer_state.read() != AnswerState::Waiting;

    let config_for_results = config.clone();

    rsx! {
        div { class: "screen game-screen",
            // Header
            div { class: "game-header",
                button {
                    class: "back-btn",
                    onclick: move |_| { history.write().clear(); current_screen.set(Screen::Home); },
                    "\u{2715}"
                }
                div { class: "progress-info",
                    span { class: "progress-text", "{idx + 1} / {q_count}" }
                }
                div { class: "timer-display",
                    span { class: "timer-text", "{mins:02}:{sec_part:02}" }
                }
            }

            div { class: "progress-bar-container",
                div { class: "progress-bar-fill", style: "width: {progress_pct}%" }
            }

            div { class: "score-row",
                span { class: "score-badge", "\u{2713} {sc}" }
            }

            if *show_penalty.read() {
                div { class: "penalty-popup",
                    span { class: "penalty-popup-text", "+{penalty_display}s" }
                }
            }

            // Question
            div { class: "question-card",
                p { class: "question-label", "{question.prompt_label}" }
                div { class: if is_math { "question-prompt question-prompt--math" } else { "question-prompt" },
                    span { class: "prompt-text", "{question.prompt}" }
                }
            }

            // Answer area
            div { class: "answer-area",
                if is_typing {
                    {render_typing_input(
                        &question, is_answered,
                        typing_input, paused, accumulated_secs, start_ms,
                        score, penalty_total, mistakes, show_penalty, answer_state,
                    )}
                } else {
                    {render_multiple_choice(
                        &question, is_answered, is_math,
                        paused, accumulated_secs, start_ms,
                        score, penalty_total, mistakes, show_penalty, answer_state,
                    )}
                }

                // Feedback + post-answer actions
                if is_answered {
                    div { class: "feedback-section",
                        match answer_state.read().clone() {
                            AnswerState::Correct => rsx! {
                                div { class: "feedback feedback--correct",
                                    span { "\u{2713} Spr\u{00e1}vn\u{011b}!" }
                                }
                            },
                            AnswerState::Wrong { correct_answer: ca } => rsx! {
                                div { class: "feedback feedback--wrong",
                                    span { class: "feedback-wrong-header",
                                        span { "\u{2717} \u{0160}patn\u{011b}" }
                                        span { class: "feedback-penalty", "+{penalty_display}s" }
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
                                        progress.write().mark_known(&known_key_mark);
                                        crate::state::save_progress(&progress.read());
                                        marked_known.set(true);
                                    },
                                    "\u{2713} Toto u\u{017e} um\u{00ed}m"
                                }
                            } else if *marked_known.read() {
                                div { class: "known-badge", "Ozna\u{010d}eno \u{2713}" }
                            }

                            button {
                                class: "next-btn",
                                onclick: move |_| {
                                    if is_last {
                                        go_to_results(
                                            &config_for_results,
                                            *accumulated_secs.read(),
                                            *penalty_total.read(),
                                            *score.read(),
                                            *mistakes.read(),
                                            q_count,
                                            &mut progress,
                                            &mut current_screen,
                                            &mut history,
                                        );
                                    } else {
                                        question_index += 1;
                                        answer_state.set(AnswerState::Waiting);
                                        typing_input.set(String::new());
                                        marked_known.set(false);
                                        show_penalty.set(false);
                                        resume_timer(&mut paused, &mut start_ms);
                                    }
                                },
                                if is_last { "Dokon\u{010d}it \u{2192}" } else { "Dal\u{0161}\u{00ed} \u{2192}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Sub-render functions ────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn render_typing_input(
    question: &Question,
    is_answered: bool,
    mut typing_input: Signal<String>,
    paused: Signal<bool>,
    accumulated_secs: Signal<f64>,
    start_ms: Signal<f64>,
    score: Signal<usize>,
    penalty_total: Signal<f64>,
    mistakes: Signal<usize>,
    show_penalty: Signal<bool>,
    answer_state: Signal<AnswerState>,
) -> Element {
    let correct_key = question.correct_answer.clone();
    let correct_submit = question.correct_answer.clone();

    rsx! {
        div { class: "typing-section",
            input {
                class: "type-input",
                r#type: "text",
                placeholder: "Napi\u{0161} odpov\u{011b}\u{010f}...",
                value: "{typing_input}",
                disabled: is_answered,
                oninput: move |e| typing_input.set(e.value()),
                onkeydown: move |e: KeyboardEvent| {
                    if e.key() == Key::Enter && !is_answered {
                        let is_correct = check_typed_answer(&typing_input.read(), &correct_key);
                        submit_answer(
                            is_correct, &correct_key,
                            paused, accumulated_secs, start_ms,
                            score, penalty_total, mistakes, show_penalty, answer_state,
                        );
                    }
                }
            }
            if !is_answered {
                button {
                    class: "submit-btn",
                    onclick: move |_| {
                        let is_correct = check_typed_answer(&typing_input.read(), &correct_submit);
                        submit_answer(
                            is_correct, &correct_submit,
                            paused, accumulated_secs, start_ms,
                            score, penalty_total, mistakes, show_penalty, answer_state,
                        );
                    },
                    "Odeslat"
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_multiple_choice(
    question: &Question,
    is_answered: bool,
    is_math: bool,
    paused: Signal<bool>,
    accumulated_secs: Signal<f64>,
    start_ms: Signal<f64>,
    score: Signal<usize>,
    penalty_total: Signal<f64>,
    mistakes: Signal<usize>,
    show_penalty: Signal<bool>,
    answer_state: Signal<AnswerState>,
) -> Element {
    let grid_class = if is_math { "choices-grid choices-grid--math" } else { "choices-grid" };

    rsx! {
        div { class: "{grid_class}",
            for i in 0..question.choices.len() {
                {
                    let choice_text = question.choices[i].clone();
                    let is_correct_choice = i == question.correct_index;
                    let correct_answer_mc = question.correct_answer.clone();
                    let ans = answer_state.read().clone();

                    let base_class = match &ans {
                        AnswerState::Waiting => "choice-btn",
                        AnswerState::Correct if is_correct_choice => "choice-btn choice-btn--correct",
                        AnswerState::Wrong { .. } if is_correct_choice => "choice-btn choice-btn--reveal",
                        _ => "choice-btn choice-btn--dimmed",
                    };
                    let class = if is_math {
                        format!("{base_class} choice-btn--math")
                    } else {
                        base_class.to_string()
                    };

                    rsx! {
                        button {
                            key: "{i}",
                            class: "{class}",
                            disabled: is_answered,
                            onclick: move |_| {
                                if !is_answered {
                                    submit_answer(
                                        is_correct_choice, &correct_answer_mc,
                                        paused, accumulated_secs, start_ms,
                                        score, penalty_total, mistakes, show_penalty, answer_state,
                                    );
                                }
                            },
                            "{choice_text}"
                        }
                    }
                }
            }
        }
    }
}
