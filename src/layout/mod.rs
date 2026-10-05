//! Page layout specifications and calendar sheet compilation.

use crate::{
    blank::BlankCalendarSpec,
    calendar::{DayCell, GridMode, GridOptions, MonthModel, WeekStart, Weekday},
    error::PaperGridError,
    paper::{Margins, Millimeters, Orientation, Paper, PaperSize, Rect},
};

/// A high-level, validated layout configuration for civil months.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct LayoutSpec {
    paper: PaperSize,
    orientation: Orientation,
    margins: Margins,
    week_start: WeekStart,
    pub(crate) grid_mode: GridMode,
    adjacent_days: bool,
    header_height: Millimeters,
    weekday_height: Millimeters,
    notes_height: Option<Millimeters>,
    footer_height: Millimeters,
    cell_gap: Millimeters,
}

impl LayoutSpec {
    /// Starts a builder with A4 portrait, 12 mm margins, and natural month rows.
    pub fn builder() -> LayoutSpecBuilder {
        LayoutSpecBuilder::default()
    }
    /// Returns this layout's paper size.
    pub const fn paper(&self) -> PaperSize {
        self.paper
    }
    /// Returns this layout's orientation.
    pub const fn orientation(&self) -> Orientation {
        self.orientation
    }
    /// Returns configured margins.
    pub const fn margins(&self) -> Margins {
        self.margins
    }
    /// Returns the first weekday.
    pub const fn week_start(&self) -> WeekStart {
        self.week_start
    }
    /// Returns grid row policy.
    pub const fn grid_mode(&self) -> GridMode {
        self.grid_mode
    }
    /// Returns whether adjacent-month day numbers are included.
    pub const fn adjacent_days(&self) -> bool {
        self.adjacent_days
    }
}

/// Builder for [`LayoutSpec`].
#[derive(Debug, Clone)]
pub struct LayoutSpecBuilder {
    paper: PaperSize,
    orientation: Orientation,
    margins_mm: [f64; 4],
    week_start: WeekStart,
    grid_mode: GridMode,
    adjacent_days: bool,
    header_height_mm: f64,
    weekday_height_mm: f64,
    notes_height_mm: Option<f64>,
    footer_height_mm: f64,
    cell_gap_mm: f64,
}

impl Default for LayoutSpecBuilder {
    fn default() -> Self {
        Self {
            paper: Paper::A4.size(),
            orientation: Orientation::Portrait,
            margins_mm: [12.0; 4],
            week_start: WeekStart::Monday,
            grid_mode: GridMode::Natural,
            adjacent_days: true,
            header_height_mm: 22.0,
            weekday_height_mm: 9.0,
            notes_height_mm: None,
            footer_height_mm: 0.0,
            cell_gap_mm: 0.5,
        }
    }
}

impl LayoutSpecBuilder {
    /// Selects a built-in paper format.
    pub fn paper(mut self, paper: Paper) -> Self {
        self.paper = paper.size();
        self
    }
    /// Selects a custom page size.
    pub fn paper_size(mut self, paper: PaperSize) -> Self {
        self.paper = paper;
        self
    }
    /// Selects page orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
    /// Sets top, right, bottom, and left margins in millimeters.
    pub fn margins_mm(mut self, top: f64, right: f64, bottom: f64, left: f64) -> Self {
        self.margins_mm = [top, right, bottom, left];
        self
    }
    /// Sets equal margins in millimeters.
    pub fn uniform_margins_mm(mut self, value: f64) -> Self {
        self.margins_mm = [value; 4];
        self
    }
    /// Selects the first weekday column.
    pub fn week_start(mut self, week_start: WeekStart) -> Self {
        self.week_start = week_start;
        self
    }
    /// Selects natural or fixed six-week rows.
    pub fn grid_mode(mut self, mode: GridMode) -> Self {
        self.grid_mode = mode;
        self
    }
    /// Selects six rows when true and natural rows otherwise.
    pub fn fixed_six_weeks(self, enabled: bool) -> Self {
        self.grid_mode(if enabled {
            GridMode::FixedSixWeeks
        } else {
            GridMode::Natural
        })
    }
    /// Shows or hides day numbers in adjacent-month cells.
    pub fn adjacent_days(mut self, enabled: bool) -> Self {
        self.adjacent_days = enabled;
        self
    }
    /// Sets title/header region height in millimeters.
    pub fn header_height_mm(mut self, value: f64) -> Self {
        self.header_height_mm = value;
        self
    }
    /// Sets weekday-heading region height in millimeters.
    pub fn weekday_height_mm(mut self, value: f64) -> Self {
        self.weekday_height_mm = value;
        self
    }
    /// Adds a notes area of the given height, or removes it with `None`.
    pub fn notes_height_mm(mut self, value: Option<f64>) -> Self {
        self.notes_height_mm = value;
        self
    }
    /// Sets footer height in millimeters.
    pub fn footer_height_mm(mut self, value: f64) -> Self {
        self.footer_height_mm = value;
        self
    }
    /// Sets equal horizontal and vertical gaps between cells in millimeters.
    pub fn cell_gap_mm(mut self, value: f64) -> Self {
        self.cell_gap_mm = value;
        self
    }

    /// Validates all dimensions and builds the layout.
    pub fn build(self) -> Result<LayoutSpec, PaperGridError> {
        let margins = Margins::from_mm(
            self.margins_mm[0],
            self.margins_mm[1],
            self.margins_mm[2],
            self.margins_mm[3],
        )?;
        let non_negative = |value, name| {
            Millimeters::non_negative(value).map_err(|_| PaperGridError::InvalidMeasurement(name))
        };
        let header_height = non_negative(self.header_height_mm, "header height")?;
        let weekday_height = non_negative(self.weekday_height_mm, "weekday height")?;
        let notes_height = self
            .notes_height_mm
            .map(|v| non_negative(v, "notes height"))
            .transpose()?;
        let footer_height = non_negative(self.footer_height_mm, "footer height")?;
        let cell_gap = non_negative(self.cell_gap_mm, "cell gap")?;
        // The selected paper size has already passed positive finite validation.
        let page = self.paper.oriented(self.orientation);
        let printable_width = page.width_mm() - margins.left.get() - margins.right.get();
        let printable_height = page.height_mm() - margins.top.get() - margins.bottom.get();
        if printable_width <= 0.0 || printable_height <= 0.0 {
            return Err(PaperGridError::InvalidMargins);
        }
        if printable_height
            <= header_height.get()
                + weekday_height.get()
                + notes_height.map_or(0.0, Millimeters::get)
                + footer_height.get()
        {
            return Err(PaperGridError::InsufficientPrintableArea);
        }
        Ok(LayoutSpec {
            paper: self.paper,
            orientation: self.orientation,
            margins,
            week_start: self.week_start,
            grid_mode: self.grid_mode,
            adjacent_days: self.adjacent_days,
            header_height,
            weekday_height,
            notes_height,
            footer_height,
            cell_gap,
        })
    }
}

/// Whether a compiled page has dated month semantics or an undated planner grid.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SheetKind {
    /// A Gregorian civil month.
    Month {
        /// Civil year.
        year: u16,
        /// Civil month from one through twelve.
        month: u8,
    },
    /// A blank, date-free grid.
    Blank {
        /// Number of grid rows.
        rows: usize,
        /// Number of grid columns.
        columns: usize,
    },
}

/// Geometry for one grid cell and its optional civil-date information.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct CellLayout {
    /// Zero-based grid row.
    pub row: usize,
    /// Zero-based grid column.
    pub column: usize,
    /// Physical bounds in millimeters.
    pub bounds: Rect,
    /// Date placement for calendar sheets, absent for blank sheets.
    pub date: Option<DayCell>,
}

/// Weekday label and its measured header rectangle.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct WeekdayLayout {
    /// Weekday displayed in this heading.
    pub weekday: Weekday,
    /// Heading rectangle aligned to its grid column.
    pub bounds: Rect,
}

/// Measured grid rectangle and its row/column placements.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct GridLayout {
    /// Grid area excluding outer margins and all other page regions.
    pub bounds: Rect,
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub columns: usize,
    /// Row-major cell placements.
    pub cells: Vec<CellLayout>,
}

/// Complete, presentation-independent page geometry for one sheet.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct CalendarSheet {
    /// Calendar or blank-sheet identity.
    pub kind: SheetKind,
    /// Display title, if the selected specification has one.
    pub title: Option<String>,
    /// Oriented physical paper dimensions.
    pub paper: PaperSize,
    /// Page orientation.
    pub orientation: Orientation,
    /// Entire physical page rectangle.
    pub page_bounds: Rect,
    /// Area remaining after the four page margins.
    pub printable_bounds: Rect,
    /// Optional title region.
    pub title_bounds: Option<Rect>,
    /// Weekday headings, aligned with the grid when present.
    pub weekday_headers: Vec<WeekdayLayout>,
    /// Grid cells and their measured geometry.
    pub grid: GridLayout,
    /// Optional notes rectangle.
    pub notes_bounds: Option<Rect>,
    /// Optional footer rectangle.
    pub footer_bounds: Option<Rect>,
}

struct SheetContent {
    kind: SheetKind,
    title: Option<String>,
    rows: usize,
    columns: usize,
    dates: Vec<Option<DayCell>>,
    show_weekdays: bool,
    notes_height: Option<Millimeters>,
}

/// Compiles a Gregorian month model and page specification into print geometry.
pub fn compile_month(
    year: u16,
    month: u8,
    layout: &LayoutSpec,
) -> Result<CalendarSheet, PaperGridError> {
    let model = MonthModel::new(year, month)?;
    let day_cells = model.grid(GridOptions {
        week_start: layout.week_start,
        mode: layout.grid_mode,
        adjacent_days: layout.adjacent_days,
    });
    let rows = day_cells.len() / 7;
    compile_sheet(
        layout,
        SheetContent {
            kind: SheetKind::Month { year, month },
            title: Some(format!("{} {year}", month_name(month))),
            rows,
            columns: 7,
            dates: day_cells.into_iter().map(Some).collect(),
            show_weekdays: true,
            notes_height: layout.notes_height,
        },
    )
}

/// Compiles a date-free blank planner page.
pub fn compile_blank(blank: &BlankCalendarSpec) -> Result<CalendarSheet, PaperGridError> {
    let count = blank.rows() * blank.columns();
    let mut cells = Vec::with_capacity(count);
    for _ in 0..count {
        cells.push(None);
    }
    compile_sheet(
        blank.layout(),
        SheetContent {
            kind: SheetKind::Blank {
                rows: blank.rows(),
                columns: blank.columns(),
            },
            title: blank.title().map(ToOwned::to_owned),
            rows: blank.rows(),
            columns: blank.columns(),
            dates: cells,
            show_weekdays: blank.weekday_header(),
            notes_height: blank.layout().notes_height,
        },
    )
}

fn compile_sheet(
    layout: &LayoutSpec,
    content: SheetContent,
) -> Result<CalendarSheet, PaperGridError> {
    let SheetContent {
        kind,
        title,
        rows,
        columns,
        dates,
        show_weekdays,
        notes_height,
    } = content;
    if rows == 0 || columns == 0 || dates.len() != rows * columns {
        return Err(PaperGridError::InvalidGridDimensions { rows, columns });
    }
    let paper = layout.paper.oriented(layout.orientation);
    let page = Rect::new(0.0, 0.0, paper.width_mm(), paper.height_mm())?;
    let m = layout.margins;
    let printable = Rect::new(
        m.left.get(),
        m.top.get(),
        paper.width_mm() - m.left.get() - m.right.get(),
        paper.height_mm() - m.top.get() - m.bottom.get(),
    )?;
    let title_rect = (layout.header_height.get() > 0.0).then(|| Rect {
        x: printable.x,
        y: printable.y,
        width: printable.width,
        height: layout.header_height.get(),
    });
    let weekday_y = printable.y + layout.header_height.get();
    let weekday_rect_height = if show_weekdays {
        layout.weekday_height.get()
    } else {
        0.0
    };
    let notes_h = notes_height.map_or(0.0, Millimeters::get);
    let footer_h = layout.footer_height.get();
    let grid_y = weekday_y + weekday_rect_height;
    let grid_bottom = printable.bottom() - notes_h - footer_h;
    let grid_bounds = Rect::new(printable.x, grid_y, printable.width, grid_bottom - grid_y)?;
    let gap = layout.cell_gap.get();
    let inner_width = grid_bounds.width - gap * (columns - 1) as f64;
    let inner_height = grid_bounds.height - gap * (rows - 1) as f64;
    if inner_width <= 0.0 || inner_height <= 0.0 {
        return Err(PaperGridError::InsufficientPrintableArea);
    }
    let base_width = inner_width / columns as f64;
    let base_height = inner_height / rows as f64;
    let mut placed = Vec::with_capacity(dates.len());
    for row in 0..rows {
        let y = grid_bounds.y + row as f64 * (base_height + gap);
        let height = if row + 1 == rows {
            grid_bounds.bottom() - y
        } else {
            base_height
        };
        for column in 0..columns {
            let x = grid_bounds.x + column as f64 * (base_width + gap);
            let width = if column + 1 == columns {
                grid_bounds.right() - x
            } else {
                base_width
            };
            placed.push(CellLayout {
                row,
                column,
                bounds: Rect::new(x, y, width, height)?,
                date: dates[row * columns + column],
            });
        }
    }
    let weekdays = if show_weekdays && columns == 7 && weekday_rect_height > 0.0 {
        let header = Rect::new(printable.x, weekday_y, printable.width, weekday_rect_height)?;
        let inner = header.width - gap * 6.0;
        let w = inner / 7.0;
        (0..7)
            .map(|col| {
                let x = header.x + col as f64 * (w + gap);
                let width = if col == 6 { header.right() - x } else { w };
                WeekdayLayout {
                    weekday: layout.week_start.weekday_at(col),
                    bounds: Rect {
                        x,
                        y: header.y,
                        width,
                        height: header.height,
                    },
                }
            })
            .collect()
    } else {
        Vec::new()
    };
    let notes_bounds = (notes_h > 0.0).then(|| Rect {
        x: printable.x,
        y: grid_bounds.bottom(),
        width: printable.width,
        height: notes_h,
    });
    let footer_bounds = (footer_h > 0.0).then(|| Rect {
        x: printable.x,
        y: printable.bottom() - footer_h,
        width: printable.width,
        height: footer_h,
    });
    Ok(CalendarSheet {
        kind,
        title,
        paper,
        orientation: layout.orientation,
        page_bounds: page,
        printable_bounds: printable,
        title_bounds: title_rect,
        weekday_headers: weekdays,
        grid: GridLayout {
            bounds: grid_bounds,
            rows,
            columns,
            cells: placed,
        },
        notes_bounds,
        footer_bounds,
    })
}

fn month_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        _ => "December",
    }
}
