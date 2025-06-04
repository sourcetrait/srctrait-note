use std::{borrow::Cow, path::Path, process::ExitCode};
use clap::Parser;
use clapx::styl::srctrait::*;
use clapx::subcmd::cli::CliCommand;
use srctrait_common_tomlx::starter::trim_starter_toml_file_comments;
use tomlx::ToStarterToml;
use crate::*;

pub fn run() -> ExitCode {
    match run_cli() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            let source = e.source()
                .map_or(String::new(), |s| format!("\n       {s}"));

            eprintln!("{STYL_ERROR}error:{STYL_ERROR:#} {e}{source}");
            ExitCode::FAILURE
        }
    }
}

fn run_cli() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Cli(cmd) => match cmd {
            CliCommand::Config(_cmd) => run_config(),
            CliCommand::Alias(_cmd) => todo!(),
        },
        Command::Today(cmd) => match &cmd.from {
            Some(TodaySubCommand::From{when}) => run_today_from(when),
            None => run_today()
        },
        Command::Yesterday => run_yesterday(),
        Command::Day{when} => run_day(when),
        Command::Idea{topic} => run_idea(topic),
        Command::Todo{topic} => run_todo(topic),
        Command::Plan{topic} => run_plan(topic),
        Command::Pick{kind} => run_pick(kind),
    }
}

fn init_user() -> anyhow::Result<(lib::NoteConfig, lib::NotesDir)> {
    let config = user_notes_config()?;
    let dir = user_notes_dir(&config)?;
    dir.init()?;
    Ok((config, dir))
}

fn run_config() -> anyhow::Result<()> {
    let (config, _notes_dir) = init_user()?;
    let config_file = note_config_file()?;
    let first_run = !config_file.is_file();

    if first_run {
        config.to_starter_toml_file(&config_file, tomlx::starter::GENERIC_INTRO)?;
    }

    run_editor(&config, &config_file)?;

    if first_run {
        trim_starter_toml_file_comments(&config_file)?;
    }

    Ok(())
}

fn run_today() -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let note = lib::Note::new(lib::NoteType::Today(lib::Date::now()));
    let note_file = lib::note_for_date(&notes_dir, &note, None)?;
    run_editor(&config, &note_file)
}

fn run_today_from(when: &str) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let today = lib::Date::now();
    let when = today.from(when)?;
    let note = lib::Note::new(lib::NoteType::Today(today));
    let note_file = lib::note_for_date(&notes_dir, &note, Some(when))?;
    run_editor(&config, &note_file)
}

fn run_day(when: String) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let note = lib::Note::new(lib::NoteType::Today(lib::Date::now().from(&when)?));
    let note_file = lib::note_for_date(&notes_dir, &note, None)?;
    run_editor(&config, &note_file)
}

fn run_yesterday() -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let note = lib::Note::new(lib::NoteType::Today(lib::Date::now().from("yesterday")?));
    let note_file = lib::note_for_date(&notes_dir, &note, None)?;
    run_editor(&config, &note_file)
}

fn run_idea(topic: String) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let note = lib::Note::new(lib::NoteType::Idea(topic.to_string()));
    let note_file = lib::note_for_topic(&notes_dir, &note)?;
    run_editor(&config, &note_file)
}

fn run_todo(topic: String) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let note = lib::Note::new(lib::NoteType::Todo(topic.to_string()));
    let note_file = lib::note_for_topic(&notes_dir, &note)?;
    run_editor(&config, &note_file)
}

fn run_plan(topic: Option<String>) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let note = lib::Note::new(lib::NoteType::Plan(topic));
    let note_file = lib::note_for_optional_topic(&notes_dir, &note)?;
    run_editor(&config, &note_file)
}

fn run_pick(kind: Option<CliNoteKind>) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let kind: Option<lib::NoteKind> = kind.map(|k| k.into());
    let dir: Cow<'_, Path> = if let Some(kind) = kind {
        Cow::Owned(notes_dir.kind_dir(kind))
    } else {
        Cow::Borrowed(&notes_dir)
    };
    
    if let Some(note_path) = run_picker(&config, &dir)? {
        lib::Note::from_filepath(&notes_dir, &note_path)?;
        run_editor(&config, &note_path)
    } else {
        println!("Cancelled.");
        Ok(())
    }
}
