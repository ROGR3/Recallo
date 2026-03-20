You are working on **Recallo**, an Android vocabulary training app built with **Dioxus 0.7** (Rust) targeting mobile via WebView.

## Project overview

- Korean vocabulary quiz app (more languages may come later)
- ~480 words classified into 8 categories: Noun, Verb, Adjective, Adverb, Expression, Grammar, Loanword, Food
- Game modes: 10/20 words (normal), 10/20 words (perfect — ends on first mistake)
- Multiple choice (90%) and typing (10%) questions, bidirectional (Korean↔English)
- Tracks known/unknown words and best times per mode+category in local JSON
- Persistence via JNI `Context.getFilesDir()` on Android, `~/.config/recallo/` on desktop

## Tech stack

- **Rust** + **Dioxus 0.7** (mobile feature, renders via Android WebView)
- `serde`/`serde_json` for persistence, `rand` for randomization
- `async-std` for native async, `gloo-timers`/`js-sys` for wasm
- `ndk-context` + `jni` for Android file path resolution
- CSS in `assets/main.css` with `prefers-color-scheme` auto light/dark
- `dx` CLI for building, `scripts/android-patch.sh` for post-build fixes

## Project structure

```
src/
  main.rs              — app entry, asset declarations, screen router
  data/
    words.rs           — static word database (~480 entries with categories)
    mod.rs
  screens/
    home.rs            — language selection
    category.rs        — category picker + mastery stats + word filters
    mode_select.rs     — game mode picker (10/20, normal/perfect) + best times
    game.rs            — quiz engine (MC + typing, timer, "I know this")
    results.rs         — score, time, personal best display
    mod.rs
  state/
    app_state.rs       — Screen enum, Language, GameMode, GameConfig, GameResult
    persistence.rs     — load/save JSON progress, Android JNI file path
    mod.rs
assets/
  main.css             — full design system
  logo.png             — app logo (used in-app)
scripts/
  android-patch.sh     — post-dx-build: patches SDK version, icons, re-runs gradle
.cargo/
  config.toml          — 16KB page alignment linker flags for Android arm64
```

## Build & verify

The Justfile defines all build recipes. **When verifying changes, `cargo check` is sufficient and preferred.** Do NOT install to a physical device — the user will do that manually.

```bash
cargo check              # fast compilation check (USE THIS to verify changes)
cargo clippy             # lint check
just build               # full Android APK build (slow, includes dx + gradle)
just install             # build + install on device (DO NOT run unless asked)
```

## Key Dioxus 0.7 quirks

- `dx` hardcodes `targetSdk=33` and `compileSdk=33` — patched post-build to 35
- `dx` overwrites launcher icons every build — patched post-build by `android-patch.sh`
- `Dioxus.toml` `[android]` section is mostly ignored by dx 0.7
- Assets are declared with `const X: Asset = asset!("/assets/file.ext")` and referenced via `document::Stylesheet` or `img { src: X }`
- State is managed via `Signal<T>` passed as component props, not global context
- The `asset!()` macro path is relative to the project root with a leading `/`

## Word data

Words live in `src/data/words.rs` as a `pub static WORDS: &[Word]` array. The `/import-words` opencode command can regenerate this from a `dump.txt` file. When editing words manually, keep the existing format and category grouping.

## Important conventions

- English translations: no redundant synonyms (e.g. "pants" not "pants / trousers") unless meanings are genuinely different (e.g. "snow / eye" for 눈)
- Timer pauses when an answer is selected, resumes on "Next"
- Perfect mode: wrong answer → immediate game over, result not recorded as best time
- Progress survives `adb install -r` (reinstall) but is lost on full uninstall
