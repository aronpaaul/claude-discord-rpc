use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActivityKind {
    Playing,
    Listening,
    Watching,
    Competing,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum StatusField {
    Name,
    State,
    Details,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TimerMode {
    Hidden,
    Session,
    Task,
    ServiceStart,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenStyle {
    Compact,
    Full,
    Spaced,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModelStyle {
    Pretty,
    Family,
    RawId,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SessionPick {
    BusyFirst,
    MostRecent,
    NameFilter,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Currency {
    Usd,
    Rub,
    Eur,
}
