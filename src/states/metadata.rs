use super::*;
use crate::op_cli::OnePassword;
use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, SelectableList, Widget};

pub struct MetadataScreen {
    pub op: OnePassword,
    pub vault_index: usize,
    pub index: usize,
}

impl From<vaults::VaultScreen> for MetadataScreen {
    fn from(vault: vaults::VaultScreen) -> Self {
        Self {
            op: vault.op,
            index: 0,
            vault_index: vault.index,
        }
    }
}

impl From<item::ItemScreen> for MetadataScreen {
    fn from(item: item::ItemScreen) -> Self {
        Self {
            op: item.op,
            index: item.index,
            vault_index: item.vault_index,
        }
    }
}

impl MetadataScreen {
    pub fn handle_key(self, key: Key) -> Screen {
        match key {
            Key::Char('q') => panic!(),
            Key::Char('j') => self.next_item(),
            Key::Char('k') => self.previous_item(),
            Key::Char('\t') => Screen::Vault(self.into()),
            Key::Char('\n') => Screen::Item(self.into()),
            _ => Screen::Metadata(self),
        }
    }

    fn next_item(self) -> Screen {
        let index = if self.index == self.op.metadata_in_vault(self.vault_index).len() - 1 {
            0
        } else {
            self.index + 1
        };
        Screen::Metadata(MetadataScreen {
            op: self.op,
            index,
            vault_index: self.vault_index,
        })
    }

    fn previous_item(self) -> Screen {
        let index = if self.index == 0 {
            self.op.metadata_in_vault(self.vault_index).len() - 1
        } else {
            self.index - 1
        };
        Screen::Metadata(MetadataScreen {
            op: self.op,
            index,
            vault_index: self.vault_index,
        })
    }

    pub fn render(&self, mut frame: TermFrame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(frame.size());

        let highlight_style = Style::default()
            .fg(Color::LightGreen)
            .modifier(Modifier::BOLD);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Vaults"))
            .items(&self.op.vaults)
            .select(Some(self.vault_index))
            .highlight_style(highlight_style)
            .render(&mut frame, chunks[0]);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Items"))
            .items(&self.op.metadata_in_vault(self.vault_index))
            .select(Some(self.index))
            .highlight_style(highlight_style)
            .highlight_symbol(">")
            .render(&mut frame, chunks[1]);
    }
}
