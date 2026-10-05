//! Date-free planner grids and their validated builder.
//!
//! To compare a generated blank sheet with a human-facing reference, see the
//! [Beta Calendars blank calendar](https://www.betacalendars.com/blank-calendar).

use crate::{
    GridMode,
    calendar::WeekStart,
    error::PaperGridError,
    layout::LayoutSpec,
    paper::{Orientation, Paper, PaperSize},
};

/// Configuration for compiling an undated planner sheet.
#[derive(Debug, Clone, PartialEq)]
pub struct BlankCalendarSpec {
    layout: LayoutSpec,
    rows: usize,
    columns: usize,
    weekday_header: bool,
    title: Option<String>,
}

impl BlankCalendarSpec {
    /// Starts a blank-sheet builder with A4 portrait, six rows, and seven columns.
    pub fn builder() -> BlankCalendarSpecBuilder {
        BlankCalendarSpecBuilder::default()
    }
    /// Returns the page and region configuration.
    pub const fn layout(&self) -> &LayoutSpec {
        &self.layout
    }
    /// Returns the configured number of rows.
    pub const fn rows(&self) -> usize {
        self.rows
    }
    /// Returns the configured number of columns.
    pub const fn columns(&self) -> usize {
        self.columns
    }
    /// Returns whether weekday headings are included.
    pub const fn weekday_header(&self) -> bool {
        self.weekday_header
    }
    /// Returns the optional title printed in the header region.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

/// Builder for [`BlankCalendarSpec`].
#[derive(Debug, Clone)]
pub struct BlankCalendarSpecBuilder {
    layout: crate::layout::LayoutSpecBuilder,
    rows: usize,
    columns: usize,
    weekday_header: bool,
    title: Option<String>,
}

impl Default for BlankCalendarSpecBuilder {
    fn default() -> Self {
        Self {
            layout: LayoutSpec::builder().grid_mode(GridMode::FixedSixWeeks),
            rows: 6,
            columns: 7,
            weekday_header: true,
            title: Some("Blank calendar".to_owned()),
        }
    }
}

impl BlankCalendarSpecBuilder {
    /// Selects a built-in paper size.
    pub fn paper(mut self, paper: Paper) -> Self {
        self.layout = self.layout.paper(paper);
        self
    }
    /// Selects a validated custom page size.
    pub fn paper_size(mut self, paper: PaperSize) -> Self {
        self.layout = self.layout.paper_size(paper);
        self
    }
    /// Selects page orientation.
    pub fn orientation(mut self, value: Orientation) -> Self {
        self.layout = self.layout.orientation(value);
        self
    }
    /// Sets page margins in millimeters.
    pub fn margins_mm(mut self, top: f64, right: f64, bottom: f64, left: f64) -> Self {
        self.layout = self.layout.margins_mm(top, right, bottom, left);
        self
    }
    /// Sets the number of rows; valid values are from one through 31.
    pub fn rows(mut self, value: usize) -> Self {
        self.rows = value;
        self
    }
    /// Sets the number of columns; valid values are from one through 31.
    pub fn columns(mut self, value: usize) -> Self {
        self.columns = value;
        self
    }
    /// Enables or disables the seven weekday headings.
    pub fn weekday_header(mut self, value: bool) -> Self {
        self.weekday_header = value;
        self
    }
    /// Sets the displayed title.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Removes the displayed title while keeping the configured header region.
    pub fn without_title(mut self) -> Self {
        self.title = None;
        self
    }
    /// Sets a week start for weekday headings.
    pub fn week_start(mut self, value: WeekStart) -> Self {
        self.layout = self.layout.week_start(value);
        self
    }
    /// Sets header height in millimeters.
    pub fn header_height_mm(mut self, value: f64) -> Self {
        self.layout = self.layout.header_height_mm(value);
        self
    }
    /// Sets weekday heading height in millimeters.
    pub fn weekday_height_mm(mut self, value: f64) -> Self {
        self.layout = self.layout.weekday_height_mm(value);
        self
    }
    /// Adds a notes area with the given height, or removes it when `None`.
    pub fn notes_height_mm(mut self, value: Option<f64>) -> Self {
        self.layout = self.layout.notes_height_mm(value);
        self
    }
    /// Sets the footer height in millimeters.
    pub fn footer_height_mm(mut self, value: f64) -> Self {
        self.layout = self.layout.footer_height_mm(value);
        self
    }
    /// Sets the gap between grid cells in millimeters.
    pub fn cell_gap_mm(mut self, value: f64) -> Self {
        self.layout = self.layout.cell_gap_mm(value);
        self
    }

    /// Validates and builds the blank-sheet specification.
    pub fn build(self) -> Result<BlankCalendarSpec, PaperGridError> {
        if !(1..=31).contains(&self.rows) || !(1..=31).contains(&self.columns) {
            return Err(PaperGridError::InvalidGridDimensions {
                rows: self.rows,
                columns: self.columns,
            });
        }
        if self.weekday_header && self.columns != 7 {
            return Err(PaperGridError::InvalidGridDimensions {
                rows: self.rows,
                columns: self.columns,
            });
        }
        let layout = self.layout.build()?;
        Ok(BlankCalendarSpec {
            layout,
            rows: self.rows,
            columns: self.columns,
            weekday_header: self.weekday_header,
            title: self.title,
        })
    }
}
