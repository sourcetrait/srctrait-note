use std::{fs, path::{Path, PathBuf}, process::{Command, Stdio}};

use crate::*;
use dirs;

pub const ENV_VAR_SRCTRAIT_NOTES_DIR: &'static str = "SRCTRAIT_NOTES_DIR";
pub const ENV_VAR_SRCTRAIT_CONFIG_DIR: &'static str = "SRCTRAIT_CONFIG_DIR";

const HOME_CONFIG_DIR: &'static str = ".config";

pub const NOTE_CONFIG_DIR: &'static str = "srctrait/note";
pub const CONFIG_FILENAME: &'static str = "config.toml";

pub(crate) fn user_config_dir() -> anyhow::Result<PathBuf> {
    if let Some(dir) = std::env::var_os(ENV_VAR_SRCTRAIT_CONFIG_DIR) {
        Ok(PathBuf::from(dir))
    } else {
        dirs::home_dir()
            .with_context(|| "User directory not found")
            .map(|d| d.join(HOME_CONFIG_DIR))
    }
}

pub(crate) fn note_config_file() -> anyhow::Result<PathBuf> {
    let note_config_dir = user_config_dir()?.join(NOTE_CONFIG_DIR);
    if !note_config_dir.is_dir() {
        fs::create_dir_all(&note_config_dir)
            .with_context(|| format!("Unable to create SourceTrait Note config dir: {}", note_config_dir.display()))?;
    }

    let note_config_file = note_config_dir.join(CONFIG_FILENAME);
    Ok(note_config_file)
}

pub(crate) fn user_notes_config() -> anyhow::Result<lib::NoteConfig> {
    let note_config_file = note_config_file()?;
    let input = if note_config_file.is_file() {
        lib::NoteConfigInput::from_toml(&note_config_file)?
    } else {
        lib::NoteConfigInput::default()
    };

    let config = lib::NoteConfig::from_input(
        input,
        dirs::document_dir()
            .with_context(|| format!("Unable to determine user documents directory"))?
    );

    Ok(config)
}

pub(crate) fn user_notes_dir(config: &lib::NoteConfig) -> anyhow::Result<lib::NotesDir> {
    if let Some(dir) = std::env::var_os(ENV_VAR_SRCTRAIT_NOTES_DIR) {
        let dir = shellexpand::path::full(&PathBuf::from(&dir))
            .with_context(|| format!("Failed to expand config.editor: {}", config.editor()))?
            .to_path_buf();

        Ok(lib::NotesDir::new(dir))
    } else {
        let dir = shellexpand::path::full(config.notes_dir())
            .with_context(|| format!("Failed to expand config.editor: {}", config.editor()))?
            .to_path_buf();

        Ok(lib::NotesDir::new(dir))
    }
}

pub fn run_editor(config: &lib::NoteConfig, file: &Path) -> anyhow::Result<()> {
    let editor = shellexpand::full(config.editor())
        .with_context(|| format!("Failed to expand config.editor: {}", config.editor()))?;

    let output = Command::new(&*editor)
        .arg(file)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .with_context(|| format!("Unable to run editor: `{}`", editor))?;

    if output.status.success() {
        Ok(())
    } else {
        let out = format!("{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        Err(anyhow::anyhow!("Editor failure: {editor} :: {out}"))
    }
}
