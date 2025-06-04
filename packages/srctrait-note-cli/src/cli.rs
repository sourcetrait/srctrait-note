use clap;
use srctrait_common_clapx as clapx;
use crate::*;

#[derive(Debug, clap::Parser)]
#[clap(version,about)]
#[clap(styles = clapx::CLAP_STYLE_SRCTRAIT)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Command {
    /// Edits today's note
    Today(TodayCommand),
    /// Edits yesterday's note
    #[clap(alias = "ayer")]
    Yesterday,
    /// Edits a note for the given date
    #[clap(alias = "date")]
    Day {
        /// Date of the note
        when: String,
    },
    /// Edits an idea note for a topic
    Idea {
        /// The name of the idea
        topic: String,
    },
    /// Edits a todo list for a topic
    Todo {
        /// The list name
        topic: String,
    },
    /// Edits the master plan or a topical one
    Plan {
        /// Option plan topic
        topic: Option<String>,
    },
    /// Opens a note using your preferred file picker
    Pick {
        kind: Option<CliNoteKind>,
    },
    #[clap(subcommand)]
    Cli(clapx::subcmd::cli::CliCommand)
}

#[derive(Debug, clap::Parser)]
pub(crate) struct TodayCommand {
    /// Carries over the specified day's notes into today's
    #[clap(subcommand)]
    pub(crate) from: Option<TodaySubCommand>,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum TodaySubCommand {
    /// Carries over the specified day's notes into today's
    From {
        /// Which date to carry over
        when: String
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum CliNoteKind {
    Today,
    Idea,
    Todo,
    Plan
}

impl From<CliNoteKind> for lib::NoteKind {
    fn from(value: CliNoteKind) -> Self {
        match value {
            CliNoteKind::Today => lib::NoteKind::Today,
            CliNoteKind::Idea => lib::NoteKind::Idea,
            CliNoteKind::Todo => lib::NoteKind::Todo,
            CliNoteKind::Plan => lib::NoteKind::Plan,
        }
    }
}


