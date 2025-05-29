use std::{fs, path::{Path, PathBuf}};
use derive_builder as builder;
use crate::*;

pub const NOTES_DIR_NAME: &'static str = "notes";

const EDITOR_CMD: &'static str ="$EDITOR";

#[derive(Debug, builder::Builder, Clone, serde::Serialize, serde::Deserialize)]
pub struct NoteConfig {
    /// should default to ~/Documents/notes equivalent on OS
    notes_dir: PathBuf,
    editor: String
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, builder::Builder)]
#[builder(default)]
pub struct NoteConfigInput {
    /// defaults to ~/Documents/notes equivalent on OS
    notes_dir: Option<PathBuf>,
    /// defaults to $EDITOR
    editor: Option<String>
}

impl NoteConfigInput {
    pub fn builder() -> NoteConfigInputBuilder {
        NoteConfigInputBuilder::default()
    }

    pub fn from_toml(file: &Path) -> Result<Self> {
        let content = fs::read_to_string(file)
            .map_err(|e| Error::Io(format!("Unable to read SourceTrait Note config file: {}", file.display()), e))?;

        toml::from_str(&content)
            .map_err(|e| Error::ParseToml(format!("Failed to parse SourceTrait Note config file: {}", file.display()), e))
    }
}

impl NoteConfig {
    pub fn builder() -> NoteConfigBuilder {
        NoteConfigBuilder::create_empty()
    }

    pub fn from_input(
        input: NoteConfigInput,
        user_documents_dir: PathBuf
    ) -> Self
    {
        Self {
            notes_dir: input.notes_dir
                .unwrap_or_else(|| user_documents_dir.join(NOTES_DIR_NAME)),
            editor: input.editor
                .unwrap_or_else(|| EDITOR_CMD.to_string()),
        }
    }

    pub fn to_toml_file(&self, file: &Path) -> Result<()> {
        let content = toml::to_string(&self).expect("self valid");
        fs::write(file, content)
            .map_err(|e| Error::Io(format!("Unable to write to SourceTrait Note config file: {}", file.display()), e))?;

        Ok(())
    }

    pub fn to_toml(&self) -> Result<String> {
        let content = toml::to_string(&self).expect("self valid");
        Ok(content)
    }

    pub fn notes_dir(&self) -> &Path {
        &self.notes_dir
    }

    pub fn editor(&self) -> &str {
        &self.editor
    }
}
