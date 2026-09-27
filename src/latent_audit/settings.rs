//! Latent-audit 2026-09 regression tests (TDD red phase) for `settings`.
//! Every test is `#[ignore]`d and asserts the CORRECT behaviour, so it fails
//! on the current code and passes once the corresponding bug is fixed.
//!
//! Isolation follows the existing `tests` module: each test binds the
//! manager to its own TempDir path via `with_path`, never touching HOME
//! (issue #23).

use super::*;
use tempfile::TempDir;

fn make_test_path() -> (PathBuf, TempDir) {
    let temp_dir = TempDir::new().expect("create temp_dir");
    let path = temp_dir.path().join(".kakeibon").join("KakeiBon.json");
    (path, temp_dir)
}

fn open_with_content(content: &str) -> (Result<SettingsManager, SettingsError>, TempDir) {
    let (path, temp) = make_test_path();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, content).unwrap();
    (SettingsManager::with_path(path), temp)
}

fn assert_recovers_to_defaults(content: &str) {
    let (result, _temp) = open_with_content(content);
    match result {
        Ok(manager) => assert!(
            manager.keys().is_empty(),
            "corrupt settings {:?} should fall back to defaults, got keys {:?}",
            content,
            manager.keys()
        ),
        Err(e) => panic!(
            "SettingsManager must recover from corrupt settings {:?} instead of erroring (app would fail to launch): {}",
            content, e
        ),
    }
}

/// L28 — a settings file containing `null` makes SettingsManager::with_path
/// return Err, which lib.rs setup propagates → app cannot launch.
/// Expected: fall back to default (empty) settings.
#[test]
fn latent_l28_null_settings_file_falls_back_to_defaults() {
    assert_recovers_to_defaults("null");
}

/// L28 — a settings file containing a JSON array `[]` makes
/// SettingsManager::with_path return Err → app cannot launch.
/// Expected: fall back to default (empty) settings.
#[test]
fn latent_l28_array_settings_file_falls_back_to_defaults() {
    assert_recovers_to_defaults("[]");
}

/// L28 — a truncated JSON object (e.g. from an external editor or a disk
/// error) makes SettingsManager::with_path return Err → app cannot launch.
/// Expected: fall back to default (empty) settings.
#[test]
fn latent_l28_truncated_settings_file_falls_back_to_defaults() {
    assert_recovers_to_defaults("{\n  \"language\": \"ja\",\n  \"font_si");
}

/// L28 — the unreadable file is kept as `<name>.corrupt` (so the user's
/// content is not silently lost), and a later save writes a valid file.
#[test]
fn latent_l28_corrupt_settings_file_is_backed_up_and_replaced_on_save() {
    let (path, _temp) = make_test_path();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let corrupt = "{\n  \"language\": \"ja\",\n  \"font_si";
    fs::write(&path, corrupt).unwrap();

    let mut manager = SettingsManager::with_path(path.clone()).expect("must recover");

    let mut backup = path.clone().into_os_string();
    backup.push(".corrupt");
    assert_eq!(
        fs::read_to_string(&backup).expect("corrupt copy must exist"),
        corrupt,
        "the original content must be preserved in the .corrupt copy"
    );

    manager.set("language", "en").expect("set after recovery");
    manager.save().expect("save after recovery");
    let reloaded = SettingsManager::with_path(path).expect("reload");
    assert_eq!(reloaded.get("language"), Some(&serde_json::json!("en")));
}
