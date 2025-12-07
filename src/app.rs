use crate::calendar::Calendar;
use crate::editor::Editor;
use crate::storage::DiaryStorage;
use chrono::NaiveDate;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Calendar,
    Editor,
}

pub struct App {
    pub mode: Mode,
    pub calendar: Calendar,
    pub editor: Editor,
    pub storage: DiaryStorage,
    pub should_quit: bool,
    pub status_message: String,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let storage = DiaryStorage::load()?;
        Ok(Self {
            mode: Mode::Calendar,
            calendar: Calendar::new(),
            editor: Editor::new(),
            storage,
            should_quit: false,
            status_message: String::from("Press 'q' to quit, Enter to edit, Tab to switch mode"),
        })
    }

    pub fn switch_to_editor(&mut self) {
        let content = self
            .storage
            .get_entry(&self.calendar.selected_date)
            .unwrap_or_default();
        self.editor.set_content(content);
        self.mode = Mode::Editor;
        self.status_message = String::from("Editing mode - Tab to save and return, Esc to cancel");
    }

    pub fn save_and_return_to_calendar(&mut self) {
        self.storage
            .set_entry(self.calendar.selected_date, self.editor.content.clone());

        if let Err(e) = self.storage.save() {
            self.status_message = format!("Error saving: {}", e);
        } else {
            self.status_message = String::from("Saved successfully! Press 'q' to quit");
        }

        self.mode = Mode::Calendar;
    }

    pub fn cancel_edit(&mut self) {
        self.mode = Mode::Calendar;
        self.status_message = String::from("Edit cancelled");
    }

    pub fn has_entry(&self, date: &NaiveDate) -> bool {
        self.storage.has_entry(date)
    }
}
