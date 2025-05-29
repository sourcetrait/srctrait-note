use std::{fs, process::ExitCode};
use clap::Parser;
use crate::*;

pub fn run() -> ExitCode {
    match run_cli() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            let source = e.source()
                .map_or(String::new(), |s| format!("\n       {s}"));

            eprintln!("{ERROR}error:{ERROR:#} {e}{source}");
            ExitCode::FAILURE
        }
    }
}

fn run_cli() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Config => run_config(),
        Command::Today(cmd) => match &cmd.from {
            Some(TodaySubCommand::From{when}) => run_today_from(when),
            None => run_today()
        },
        Command::Yesterday => run_yesterday(),
        Command::Day{when} => run_day(when),
        Command::Idea{topic} => run_idea(topic),
        Command::Todo{topic} => run_todo(topic),
        Command::Plan{topic} => run_plan(topic.as_deref()),
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
        let content = config.to_toml()?
            .lines()
            .map(|l| format!("#{l}"))
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&config_file, content)?;
    }

    run_editor(&config, &config_file)?;

    if first_run {
        let content = fs::read_to_string(&config_file)
            .with_context(|| format!("Unable to read config file: {}", config_file.display()))?
            .lines()
            .filter(|l| !l.starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(config_file, content)?;
    }

    Ok(())
}

fn run_today() -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let today = lib::Date::now();
    let note_file = lib::note_for_date(notes_dir, today, None)?;
    run_editor(&config, &note_file)
}

fn run_today_from(when: &str) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let today = lib::Date::now();
    let when = today.from(when)?;
    let note_file = lib::note_for_date(notes_dir, today, Some(when))?;
    run_editor(&config, &note_file)
}

fn run_day(when: &str) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let day = lib::Date::now().from(&when)?;
    let note_file = lib::note_for_date(notes_dir, day, None)?;
    run_editor(&config, &note_file)
}

fn run_yesterday() -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let day = lib::Date::now().from("yesterday")?;
    let note_file = lib::note_for_date(notes_dir, day, None)?;
    run_editor(&config, &note_file)
}

fn run_idea(topic: &str) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let note_file = lib::note_for_topic(notes_dir, lib::NoteKind::Idea, topic)?;
    run_editor(&config, &note_file)
}

fn run_todo(topic: &str) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let note_file = lib::note_for_topic(notes_dir, lib::NoteKind::Todo, topic)?;
    run_editor(&config, &note_file)
}

fn run_plan(topic: Option<&str>) -> anyhow::Result<()> {
    let (config, notes_dir) = init_user()?;
    let kind = match topic {
        Some(_) => lib::NoteKind::PlanTopic,
        None => lib::NoteKind::Plan,
    };
    
    let note_file = lib::note_for_optional_topic(notes_dir, kind, topic)?;
    run_editor(&config, &note_file)
}
