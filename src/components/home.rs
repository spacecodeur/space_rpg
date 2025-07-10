use color_eyre::Result;
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, book::Book, action_input::ActionInput};
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    book: Book,
    action_input: ActionInput,
}

impl Home {
    pub fn new() -> Self {
        Self {
            book: Book::new(),
            action_input: ActionInput::new(),
            ..Default::default()
        }
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());
        self.book.register_action_handler(tx.clone())?;
        self.action_input.register_action_handler(tx)?;
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config.clone();
        self.book.register_config_handler(config.clone())?;
        self.action_input.register_config_handler(config)?;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        // Update child components
        self.book.update(action.clone())?;
        self.action_input.update(action.clone())?;

        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // Main layout: book (90%) and action area (10%)
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(90), // Book area
                Constraint::Percentage(10), // Action input area
            ])
            .split(area);

        // Render the book component
        self.book.draw(frame, main_layout[0])?;

        // Render the action input component
        self.action_input.draw(frame, main_layout[1])?;

        Ok(())
    }
}
