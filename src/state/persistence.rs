use crate::state::Theme;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

const SAVE_FILE: &str = "recallo_progress.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Progress {
    pub known_words: HashSet<String>,
    /// Key: "<subject>:<category>:<mode>" e.g. "korean:nouns:10"
    pub best_times: HashMap<String, f64>,
    #[serde(default)]
    pub theme: Theme,
}

impl Progress {
    pub fn is_known(&self, key: &str) -> bool {
        self.known_words.contains(key)
    }

    pub fn mark_known(&mut self, key: &str) {
        self.known_words.insert(key.to_string());
    }

    pub fn unmark_known(&mut self, key: &str) {
        self.known_words.remove(key);
    }

    pub fn get_best_time(&self, key: &str) -> Option<f64> {
        self.best_times.get(key).copied()
    }

    pub fn maybe_update_best_time(&mut self, key: &str, seconds: f64) -> bool {
        let current = self.best_times.get(key).copied().unwrap_or(f64::MAX);
        if seconds < current {
            self.best_times.insert(key.to_string(), seconds);
            true
        } else {
            false
        }
    }
}

/// Get the path to save progress data.
///
/// Android: uses Context.getFilesDir() via JNI — this is the app's private
///   internal storage at /data/data/<package>/files/. It survives `adb install -r`
///   reinstalls (code updates) but is wiped on full uninstall.
///
/// Desktop/dev: ~/.config/recallo/
fn save_path() -> String {
    #[cfg(target_os = "android")]
    {
        android_files_dir()
            .map(|d| format!("{}/{}", d, SAVE_FILE))
            .unwrap_or_else(|| format!("/data/data/com.example.Recallo/files/{}", SAVE_FILE))
    }
    #[cfg(not(target_os = "android"))]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.config/recallo/{}", home, SAVE_FILE)
    }
}

/// Call Android's Context.getFilesDir() via JNI to get the correct private
/// files directory for this app. Returns None on any error (fallback used).
#[cfg(target_os = "android")]
fn android_files_dir() -> Option<String> {
    use jni::objects::JObject;
    use jni::JavaVM;

    unsafe {
        let ctx = ndk_context::android_context();
        let vm = JavaVM::from_raw(ctx.vm().cast()).ok()?;
        let mut env = vm.attach_current_thread().ok()?;
        let activity = JObject::from_raw(ctx.context().cast());

        // Call context.getFilesDir() -> java.io.File
        let files_dir = env
            .call_method(&activity, "getFilesDir", "()Ljava/io/File;", &[])
            .ok()?
            .l()
            .ok()?;

        // Call file.getAbsolutePath() -> String
        let path_jstring = env
            .call_method(&files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
            .ok()?
            .l()
            .ok()?;

        let path: String = env.get_string(&path_jstring.into()).ok()?.into();

        Some(path)
    }
}

pub fn load_progress() -> Progress {
    let path = save_path();
    // Ensure parent directory exists (needed on desktop; Android's files/ dir always exists)
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Progress::default(),
    }
}

pub fn save_progress(progress: &Progress) {
    let path = save_path();
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(progress) {
        let _ = std::fs::write(&path, json);
    }
}
