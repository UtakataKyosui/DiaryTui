use chrono::{Datelike, Local, NaiveDate};

pub struct Calendar {
    pub current_date: NaiveDate,
    pub selected_date: NaiveDate,
    pub display_months: usize,
}

impl Calendar {
    pub fn new() -> Self {
        let today = Local::now().naive_local().date();
        Self {
            current_date: today,
            selected_date: today,
            display_months: 9,
        }
    }

    pub fn move_selection(&mut self, days: i64) {
        if let Some(new_date) = self.selected_date.checked_add_signed(chrono::Duration::days(days)) {
            self.selected_date = new_date;
            self.adjust_current_month();
        }
    }

    pub fn next_month(&mut self) {
        if let Some(new_date) = self.current_date.with_day(1).and_then(|d| {
            if d.month() == 12 {
                NaiveDate::from_ymd_opt(d.year() + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(d.year(), d.month() + 1, 1)
            }
        }) {
            self.current_date = new_date;
        }
    }

    pub fn prev_month(&mut self) {
        if let Some(new_date) = self.current_date.with_day(1).and_then(|d| {
            if d.month() == 1 {
                NaiveDate::from_ymd_opt(d.year() - 1, 12, 1)
            } else {
                NaiveDate::from_ymd_opt(d.year(), d.month() - 1, 1)
            }
        }) {
            self.current_date = new_date;
        }
    }

    fn adjust_current_month(&mut self) {
        let selected_month = self.selected_date.month();
        let current_month = self.current_date.month();
        let selected_year = self.selected_date.year();
        let current_year = self.current_date.year();

        let months_diff = if selected_year == current_year {
            selected_month as i32 - current_month as i32
        } else {
            (selected_year - current_year) * 12 + selected_month as i32 - current_month as i32
        };

        if months_diff < 0 || months_diff >= self.display_months as i32 {
            self.current_date = self.selected_date.with_day(1).unwrap();
        }
    }

    pub fn get_month_days(&self, year: i32, month: u32) -> Vec<Option<NaiveDate>> {
        let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let first_weekday = first_day.weekday().num_days_from_sunday() as usize;

        let days_in_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .unwrap()
        .signed_duration_since(first_day)
        .num_days() as u32;

        let mut days = Vec::new();

        for _ in 0..first_weekday {
            days.push(None);
        }

        for day in 1..=days_in_month {
            days.push(Some(NaiveDate::from_ymd_opt(year, month, day).unwrap()));
        }

        days
    }

    pub fn get_display_months(&self) -> Vec<(i32, u32)> {
        let mut months = Vec::new();
        let mut year = self.current_date.year();
        let mut month = self.current_date.month();

        for _ in 0..self.display_months {
            months.push((year, month));

            if month == 12 {
                year += 1;
                month = 1;
            } else {
                month += 1;
            }
        }

        months
    }
}
