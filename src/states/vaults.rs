use crate::op_cli::OnePassword;
use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, SelectableList, Widget};

use super::*;

pub struct VaultScreen {
    pub op: OnePassword,
    pub index: usize,
}

impl From<metadata::MetadataScreen> for VaultScreen {
    fn from(m: metadata::MetadataScreen) -> Self {
        Self {
            op: m.op,
            index: m.vault_index,
        }
    }
}

impl VaultScreen {
    pub fn new() -> Self {
        let mut s = Self {
            index: 0,
            op: OnePassword::new(),
        };
        s.op.load_vaults();
        s.op.load_item_metadata();
        s
    }

    pub fn handle_key(self, key: Key) -> Screen {
        match key {
            // TODO: Set something like self.must_exit and propagate up to main loop
            Key::Char('q') => panic!(),
            Key::Char('j') => self.next_vault(),
            Key::Char('k') => self.previous_vault(),
            Key::Char('\t') => Screen::Metadata(self.into()),
            _ => Screen::Vault(self),
        }
    }

    fn next_vault(self) -> Screen {
        let index = if self.index == self.op.vaults.len() - 1 {
            0
        } else {
            self.index + 1
        };
        Screen::Vault(VaultScreen { op: self.op, index })
    }

    fn previous_vault(self) -> Screen {
        let index = if self.index == 0 {
            self.op.vaults.len() - 1
        } else {
            self.index - 1
        };
        Screen::Vault(VaultScreen { op: self.op, index })
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
            .select(Some(self.index))
            .highlight_style(highlight_style)
            .highlight_symbol(">")
            .render(&mut frame, chunks[0]);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Items"))
            .items(&self.op.metadata_in_vault(self.index))
            .select(None)
            .render(&mut frame, chunks[1]);
    }
}
