//! Deterministic printable calendar layouts for Rust applications and command-line tools.
//!
//! PaperGrid turns a civil month or blank planner specification into measured page
//! geometry. It keeps dates separate from instants: calculations do not read the local
//! clock, locale, timezone, environment, or network. The optional [`render`] module
//! produces self-contained SVG, and the `serde` feature makes calculated layouts
//! available as structured data.
//!
//! # Quick start
//!
//! ```
//! use betacalendars_papergrid::{compile_month, LayoutSpec, Paper, WeekStart};
//!
//! let layout = LayoutSpec::builder()
//!     .paper(Paper::A4)
//!     .week_start(WeekStart::Monday)
//!     .fixed_six_weeks(true)
//!     .build()?;
//! let sheet = compile_month(2027, 1, &layout)?;
//! assert_eq!(sheet.grid.cells.len(), 42);
//! # Ok::<(), betacalendars_papergrid::PaperGridError>(())
//! ```
//!
//! ## Project
//!
//! PaperGrid is maintained as part of [Beta Calendars](https://www.betacalendars.com/).
//! Its calendar and geometry engine is independent of that website.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod blank;
pub mod calendar;
pub mod error;
pub mod layout;
pub mod paper;
pub mod render;

pub use blank::{BlankCalendarSpec, BlankCalendarSpecBuilder};
pub use calendar::{
    CivilDate, DayCell, GridMode, GridOptions, MonthModel, MonthRelation, WeekStart, Weekday,
};
pub use error::PaperGridError;
pub use layout::{
    CalendarSheet, CellLayout, GridLayout, LayoutSpec, LayoutSpecBuilder, SheetKind, compile_blank,
    compile_month,
};
pub use paper::{Margins, Millimeters, Orientation, Paper, PaperSize, Rect};
