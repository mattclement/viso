pub mod item;
pub mod metadata;
pub mod vaults;

use termion::event::Key;

type TermFrame<'a> = tui::terminal::Frame<
    'a,
    tui::backend::TermionBackend<
        termion::screen::AlternateScreen<
            termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
        >,
    >,
>;

pub enum Screen {
    Vault(vaults::VaultScreen),
    Metadata(metadata::MetadataScreen),
    Item(item::ItemScreen),
}

impl Screen {
    pub fn new() -> Self {
        Screen::Vault(vaults::VaultScreen::new())
    }

    /// Consume the current Screen and return the next one in the state machine
    pub fn step(self, key: Key) -> Self {
        match self {
            Screen::Vault(s) => s.handle_key(key),
            Screen::Metadata(s) => s.handle_key(key),
            Screen::Item(s) => s.handle_key(key),
        }
    }

    pub fn render(&mut self, frame: TermFrame) {
        match self {
            Screen::Vault(s) => s.render(frame),
            Screen::Metadata(s) => s.render(frame),
            Screen::Item(s) => s.render(frame),
        }
    }
}
