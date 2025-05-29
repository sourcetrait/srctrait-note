
#[derive(Debug, Clone, Copy, PartialEq, Eq,
    strum::Display, strum::IntoStaticStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum NoteKind {
    Today,
    Idea,
    Todo,
    Plan,
    #[strum(to_string = "plan")]
    PlanTopic
}
