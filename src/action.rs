use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Quit,
    ClearScreen,
    Error(String),
    Help,
    NextPage,
    PreviousPage,
}
