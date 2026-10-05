//! Validated input and layout errors.

use std::fmt;

/// Errors returned while constructing dates, page specifications, or layouts.
#[derive(Debug, Clone, PartialEq)]
pub enum PaperGridError {
    /// A civil year must be in the supported range `1..=9999`.
    InvalidYear(u16),
    /// A month number must be in `1..=12`.
    InvalidMonth(u8),
    /// A day does not exist in the specified Gregorian month.
    InvalidDay {
        /// Year supplied to the constructor.
        year: u16,
        /// Month supplied to the constructor.
        month: u8,
        /// Day supplied to the constructor.
        day: u8,
    },
    /// A physical measurement is invalid.
    InvalidMeasurement(&'static str),
    /// Margins leave no positive printable rectangle.
    InvalidMargins,
    /// A blank grid's row or column count is outside `1..=31`.
    InvalidGridDimensions {
        /// Requested row count.
        rows: usize,
        /// Requested column count.
        columns: usize,
    },
    /// The page does not leave positive space for the requested layout.
    InsufficientPrintableArea,
    /// A month range is reversed or outside the supported years.
    InvalidDateRange,
}

impl fmt::Display for PaperGridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidYear(year) => write!(f, "year {year} is outside 1..=9999"),
            Self::InvalidMonth(month) => write!(f, "month {month} is outside 1..=12"),
            Self::InvalidDay { year, month, day } => write!(
                f,
                "{year:04}-{month:02}-{day:02} is not a valid Gregorian date"
            ),
            Self::InvalidMeasurement(name) => write!(f, "{name} must be finite and valid"),
            Self::InvalidMargins => f.write_str("margins exceed the physical page size"),
            Self::InvalidGridDimensions { rows, columns } => write!(
                f,
                "blank grid dimensions {rows}x{columns} must each be in 1..=31"
            ),
            Self::InsufficientPrintableArea => {
                f.write_str("page has insufficient printable area for the requested layout")
            }
            Self::InvalidDateRange => {
                f.write_str("month range must be ordered and remain within years 1..=9999")
            }
        }
    }
}

impl std::error::Error for PaperGridError {}
