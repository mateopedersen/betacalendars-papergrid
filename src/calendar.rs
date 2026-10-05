//! Gregorian civil dates and presentation-independent month grids.

use crate::error::PaperGridError;

/// A validated Gregorian date in years 1 through 9999.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct CivilDate {
    year: u16,
    month: u8,
    day: u8,
}

impl CivilDate {
    /// Creates a date after validating the year, month, and day.
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, PaperGridError> {
        if !(1..=9999).contains(&year) {
            return Err(PaperGridError::InvalidYear(year));
        }
        if !(1..=12).contains(&month) {
            return Err(PaperGridError::InvalidMonth(month));
        }
        if day == 0 || day > days_in_month(year, month) {
            return Err(PaperGridError::InvalidDay { year, month, day });
        }
        Ok(Self { year, month, day })
    }
    /// Returns the year component.
    pub const fn year(self) -> u16 {
        self.year
    }
    /// Returns the month component in `1..=12`.
    pub const fn month(self) -> u8 {
        self.month
    }
    /// Returns the day component in `1..=31`.
    pub const fn day(self) -> u8 {
        self.day
    }
    /// Returns this date's weekday.
    pub fn weekday(self) -> Weekday {
        weekday_for(self.year, self.month, self.day)
    }
}

/// Returns the number of days in a month; an invalid month returns zero.
pub const fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Returns whether a year follows the Gregorian leap-year rule.
pub const fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Weekdays ordered Monday through Sunday.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Weekday {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

impl Weekday {
    /// Returns the Monday-based index in `0..=6`.
    pub const fn number_from_monday(self) -> u8 {
        self as u8
    }
    /// Returns the Sunday-based index in `0..=6`.
    pub const fn number_from_sunday(self) -> u8 {
        (self.number_from_monday() + 1) % 7
    }
    /// Returns the English weekday label.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
        }
    }
    pub(crate) const fn from_monday_index(index: u8) -> Self {
        match index % 7 {
            0 => Self::Monday,
            1 => Self::Tuesday,
            2 => Self::Wednesday,
            3 => Self::Thursday,
            4 => Self::Friday,
            5 => Self::Saturday,
            _ => Self::Sunday,
        }
    }
}

/// First weekday used to place dates into calendar columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum WeekStart {
    /// Monday-first week.
    #[default]
    Monday,
    /// Tuesday-first week.
    Tuesday,
    /// Wednesday-first week.
    Wednesday,
    /// Thursday-first week.
    Thursday,
    /// Friday-first week.
    Friday,
    /// Saturday-first week.
    Saturday,
    /// Sunday-first week.
    Sunday,
}

impl WeekStart {
    /// Returns this weekday's Monday-based index.
    pub const fn index(self) -> u8 {
        self as u8
    }
    /// Returns the weekday at a column offset from this week start.
    pub const fn weekday_at(self, column: usize) -> Weekday {
        Weekday::from_monday_index((self.index() + (column % 7) as u8) % 7)
    }
}

/// Identifies whether a grid placement belongs to an adjacent or displayed month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum MonthRelation {
    /// A date from the preceding month.
    Previous,
    /// A date from the displayed month.
    Current,
    /// A date from the following month.
    Next,
}

/// A date placement in a month grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct DayCell {
    /// Date value, omitted when an adjacent-month label is hidden or out of range.
    pub date: Option<CivilDate>,
    /// Day number displayed in the cell, when available.
    pub day_number: Option<u8>,
    /// Weekday represented by this column.
    pub weekday: Weekday,
    /// Zero-based grid row.
    pub row: usize,
    /// Zero-based grid column.
    pub column: usize,
    /// Month containing this cell's day.
    pub relation: MonthRelation,
}

/// Selects natural-height or six-week calendar grids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum GridMode {
    /// Use the minimum number of complete weeks.
    #[default]
    Natural,
    /// Always use six complete weeks and 42 cells.
    FixedSixWeeks,
}

/// Options for generating a month grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct GridOptions {
    /// First weekday column.
    pub week_start: WeekStart,
    /// Grid row policy.
    pub mode: GridMode,
    /// Whether adjacent-month day labels should be present.
    pub adjacent_days: bool,
}

impl Default for GridOptions {
    fn default() -> Self {
        Self {
            week_start: WeekStart::Monday,
            mode: GridMode::Natural,
            adjacent_days: true,
        }
    }
}

/// Validated civil-month facts independent of paper or rendering choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonthModel {
    year: u16,
    month: u8,
}

impl MonthModel {
    /// Creates a model for a Gregorian month.
    pub fn new(year: u16, month: u8) -> Result<Self, PaperGridError> {
        CivilDate::new(year, month, 1)?;
        Ok(Self { year, month })
    }
    /// Returns this month's year.
    pub const fn year(self) -> u16 {
        self.year
    }
    /// Returns this month's number.
    pub const fn month(self) -> u8 {
        self.month
    }
    /// Returns the number of dates in this month.
    pub const fn day_count(self) -> u8 {
        days_in_month(self.year, self.month)
    }
    /// Iterates the dates in ascending order.
    pub fn days(self) -> impl Iterator<Item = CivilDate> {
        (1..=self.day_count()).map(move |day| CivilDate {
            year: self.year,
            month: self.month,
            day,
        })
    }
    /// Places this month's dates into rows of seven cells.
    pub fn grid(self, options: GridOptions) -> Vec<DayCell> {
        let first = weekday_for(self.year, self.month, 1).number_from_monday();
        let leading = (first + 7 - options.week_start.index()) % 7;
        let natural_rows = (usize::from(leading) + usize::from(self.day_count())).div_ceil(7);
        let rows = match options.mode {
            GridMode::Natural => natural_rows,
            GridMode::FixedSixWeeks => 6,
        };
        let mut cells = Vec::with_capacity(rows * 7);
        for index in 0..rows * 7 {
            let row = index / 7;
            let column = index % 7;
            let offset_day = index as i32 - i32::from(leading) + 1;
            let (relation, year, month, day) = if offset_day < 1 {
                let (year, month) = previous_month(self.year, self.month);
                let day = i32::from(days_in_month(year, month)) + offset_day;
                (MonthRelation::Previous, year, month, day as u8)
            } else if offset_day > i32::from(self.day_count()) {
                let (year, month) = next_month(self.year, self.month);
                (
                    MonthRelation::Next,
                    year,
                    month,
                    (offset_day - i32::from(self.day_count())) as u8,
                )
            } else {
                (
                    MonthRelation::Current,
                    self.year,
                    self.month,
                    offset_day as u8,
                )
            };
            let date = match relation {
                MonthRelation::Current => Some(CivilDate { year, month, day }),
                _ if options.adjacent_days => CivilDate::new(year, month, day).ok(),
                _ => None,
            };
            let day_number = if relation == MonthRelation::Current || date.is_some() {
                Some(day)
            } else {
                None
            };
            cells.push(DayCell {
                date,
                day_number,
                weekday: options.week_start.weekday_at(column),
                row,
                column,
                relation,
            });
        }
        cells
    }
}

pub(crate) fn weekday_for(year: u16, month: u8, day: u8) -> Weekday {
    let prior = u64::from(year - 1);
    let before = prior * 365 + prior / 4 - prior / 100 + prior / 400;
    let ordinal = (1..month)
        .map(|m| u64::from(days_in_month(year, m)))
        .sum::<u64>()
        + u64::from(day - 1);
    Weekday::from_monday_index(((before + ordinal) % 7) as u8)
}

pub(crate) const fn previous_month(year: u16, month: u8) -> (u16, u8) {
    if month == 1 {
        (year.saturating_sub(1), 12)
    } else {
        (year, month - 1)
    }
}
pub(crate) const fn next_month(year: u16, month: u8) -> (u16, u8) {
    if month == 12 {
        (year.saturating_add(1), 1)
    } else {
        (year, month + 1)
    }
}
