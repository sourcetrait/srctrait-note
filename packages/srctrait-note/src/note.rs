use std::{fs, path::PathBuf};
use convert_case::{Case, Casing};

use srctrait_common_chronox::DateTimeFormat;

use crate::*;

pub fn note_for_date(notes_dir: NotesDir, date: Date, from: Option<Date>) -> Result<PathBuf> {
    let mut previous_contents = None;
    if let Some(from_date) = from {
        let from_note_dir = notes_dir
            .kind_dir(NoteKind::Today)
            .join(from_date.display(DateTimeFormat::YmSlash).to_string());

        let from_file = from_note_dir
            .join(format!("note-today-{}.md", from_date.display(DateTimeFormat::YmdDash).to_string()));

        if !from_file.is_file() {
            return Err(Error::NoDayNotes(from_date));
        }

        let mut skip = true;
        previous_contents = Some(fs::read_to_string(&from_file)
            .map_err(|e| Error::Io(format!("Unable to read daily note file: {}", from_file.display()), e))?
            .lines()
            .filter(|l| {
                if skip {
                    if l.starts_with("---") {
                        skip = false;
                    }

                    false
                } else {
                    true
                }
            })
            .collect::<Vec<_>>()
            .join("\n"));
    }

    let note_dir = notes_dir
        .kind_dir(NoteKind::Today)
        .join(date.display(DateTimeFormat::YmSlash).to_string());

    if !note_dir.is_dir() {
        fs::create_dir_all(&note_dir)
            .map_err(|e| Error::Io(format!("Unable to create note directory: {}", note_dir.display()), e))?;
    }

    let note_file = note_dir
        .join(format!("today-{}.md", date.display(DateTimeFormat::YmdDash).to_string()));

    if !note_file.is_file() {
        let vars = build_template_vars(date, None);
        let template_str = render_template_str(NoteKind::Today.default_template_str(), vars);
        fs::write(&note_file, &template_str)
            .map_err(|e| Error::Io(format!("Unable to write note file: {}", note_file.display()), e))?;
    }

    if let Some(previous_contents) = previous_contents {
        let current = fs::read_to_string(&note_file)
            .map_err(|e| Error::Io(format!("Unable to read note file: {}", note_file.display()), e))?;
        let content = format!("{current}\n{previous_contents}");

        fs::write(&note_file, &content)
            .map_err(|e| Error::Io(format!("Unable to write note file: {}", note_file.display()), e))?;
    }
    
    Ok(note_file)
}

pub fn note_for_topic(notes_dir: NotesDir, kind: NoteKind, topic: &str) -> Result<PathBuf> {
    assert!(kind != NoteKind::Today);
    let date = Date::now();
    let topic_title = topic.to_case(Case::Sentence);
    let topic = topic.to_case(Case::Kebab);
    
    let note_dir = notes_dir.kind_dir(kind);

    if !note_dir.is_dir() {
        fs::create_dir_all(&note_dir)
            .map_err(|e| Error::Io(format!("Unable to create note directory: {}", note_dir.display()), e))?;
    }

    let note_file = note_dir
        .join(format!("{kind}-{topic}.md").to_string());

    if !note_file.is_file() {
        let vars = build_template_vars(date, Some(&topic_title));
        let template_str = render_template_str(kind.default_template_str(), vars);
        fs::write(&note_file, &template_str)
            .map_err(|e| Error::Io(format!("Unable to write note file: {}", note_file.display()), e))?;
    }

    Ok(note_file)
}

pub fn note_for_optional_topic(notes_dir: NotesDir, kind: NoteKind, topic: Option<&str>) -> Result<PathBuf> {
    assert!(kind == NoteKind::Plan || kind == NoteKind::PlanTopic);
    
    if let Some(topic) = topic {
        return note_for_topic(notes_dir, kind, topic);
    }
        
    let note_dir = notes_dir.kind_dir(kind);

    if !note_dir.is_dir() {
        fs::create_dir_all(&note_dir)
            .map_err(|e| Error::Io(format!("Unable to create note directory: {}", note_dir.display()), e))?;
    }

    let note_file = note_dir.join("plan.md");

    if !note_file.is_file() {
        let date = Date::now();
        let vars = build_template_vars(date, None);
        let template_str = render_template_str(kind.default_template_str(), vars);
        fs::write(&note_file, &template_str)
            .map_err(|e| Error::Io(format!("Unable to write note file: {}", note_file.display()), e))?;
    }

    Ok(note_file)
}