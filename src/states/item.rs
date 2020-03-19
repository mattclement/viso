use super::*;
use crate::op_cli::OnePassword;
use std::process::Command;
use termion::event::Key;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget};

pub struct ItemScreen {
    pub op: OnePassword,
    pub vault_index: usize,
    pub index: usize,
}

impl From<metadata::MetadataScreen> for ItemScreen {
    fn from(metadata: metadata::MetadataScreen) -> Self {
        Self {
            op: metadata.op,
            index: metadata.index,
            vault_index: metadata.vault_index,
        }
    }
}

impl ItemScreen {
    pub fn handle_key(mut self, key: Key) -> Screen {
        match key {
            Key::Char('q') => Screen::Metadata(self.into()),
            Key::Char('y') => {
                self.yank_to_clipboard();
                Screen::Item(self)
            }
            // Key::Char('j') => self.next_item(),
            // Key::Char('k') => self.previous_item(),
            _ => Screen::Item(self),
        }
    }

    pub fn yank_to_clipboard(&mut self) {
        let uuid = self.selected_uuid();
        let text = self.op.get_item(&uuid).get_password();
        Command::new("wl-copy")
            .args(&["--trim-newline", &text])
            .status()
            .expect("NOOOOOOOOOO");
    }

    fn selected_uuid(&self) -> String {
        self.op.items_in_vault(self.vault_index)[self.index]
            .uuid
            .clone()
    }

    pub fn render(&mut self, mut frame: TermFrame) {
        // First split: Left vault area, right metadata list / item area
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(frame.size());

        let right_area = chunks[1];
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(right_area);

        let vault_area = chunks[0];
        let metadata_area = right_chunks[0];
        let item_area = right_chunks[1];

        let highlight_style = Style::default()
            .fg(Color::LightGreen)
            .modifier(Modifier::BOLD);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Vaults"))
            .items(&self.op.vaults)
            .select(Some(self.vault_index))
            .highlight_style(highlight_style)
            .render(&mut frame, vault_area);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Items"))
            .items(&self.op.metadata_in_vault(self.vault_index))
            .select(Some(self.index))
            .highlight_style(highlight_style)
            .highlight_symbol(">")
            .render(&mut frame, metadata_area);
        let uuid = self.selected_uuid();
        let text = [Text::styled(
            self.op.get_item(&uuid).get_password(),
            Style::default().fg(Color::Red),
        )];
        Paragraph::new(text.iter())
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut frame, item_area);
    }
}
