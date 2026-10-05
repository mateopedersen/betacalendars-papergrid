//! Paper units, standard page sizes, orientation, margins, and rectangles.

use crate::error::PaperGridError;

/// A validated distance in millimeters.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Millimeters(f64);

impl Millimeters {
    /// Creates a finite, strictly positive millimeter value.
    pub fn new(value: f64) -> Result<Self, PaperGridError> {
        if value.is_finite() && value > 0.0 {
            Ok(Self(value))
        } else {
            Err(PaperGridError::InvalidMeasurement("length"))
        }
    }
    /// Creates a finite, non-negative millimeter value, suitable for margins and spacing.
    pub fn non_negative(value: f64) -> Result<Self, PaperGridError> {
        if value.is_finite() && value >= 0.0 {
            Ok(Self(value))
        } else {
            Err(PaperGridError::InvalidMeasurement("length"))
        }
    }
    /// Returns the value in millimeters.
    pub const fn get(self) -> f64 {
        self.0
    }
}

/// Built-in page formats with exact metric dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Paper {
    /// ISO A4 page: 210 × 297 mm.
    #[default]
    A4,
    /// North American Letter page: 215.9 × 279.4 mm.
    Letter,
    /// ISO A5 page: 148 × 210 mm.
    A5,
    /// North American Legal page: 215.9 × 355.6 mm.
    Legal,
}

impl Paper {
    /// Returns this paper format's portrait dimensions.
    pub const fn size(self) -> PaperSize {
        let (width, height) = match self {
            Self::A4 => (210.0, 297.0),
            Self::Letter => (215.9, 279.4),
            Self::A5 => (148.0, 210.0),
            Self::Legal => (215.9, 355.6),
        };
        PaperSize {
            width: Millimeters(width),
            height: Millimeters(height),
        }
    }
    /// Returns the lowercase CLI/metadata name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::A4 => "a4",
            Self::Letter => "letter",
            Self::A5 => "a5",
            Self::Legal => "legal",
        }
    }
}

/// A validated custom or built-in page size.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct PaperSize {
    #[cfg_attr(feature = "serde", serde(rename = "width_mm"))]
    width: Millimeters,
    #[cfg_attr(feature = "serde", serde(rename = "height_mm"))]
    height: Millimeters,
}

impl PaperSize {
    /// Creates a custom page size from positive millimeter dimensions.
    pub fn custom_mm(width: f64, height: f64) -> Result<Self, PaperGridError> {
        Ok(Self {
            width: Millimeters::new(width)?,
            height: Millimeters::new(height)?,
        })
    }
    /// Returns the portrait width in millimeters.
    pub const fn width_mm(self) -> f64 {
        self.width.get()
    }
    /// Returns the portrait height in millimeters.
    pub const fn height_mm(self) -> f64 {
        self.height.get()
    }
    /// Returns the dimensions swapped for landscape orientation when needed.
    pub const fn oriented(self, orientation: Orientation) -> Self {
        match orientation {
            Orientation::Portrait => self,
            Orientation::Landscape => Self {
                width: self.height,
                height: self.width,
            },
        }
    }
}

/// Page orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Orientation {
    /// Use the paper's listed width and height.
    #[default]
    Portrait,
    /// Swap the paper's listed width and height.
    Landscape,
}

/// Insets from the physical page edges, measured in millimeters.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Margins {
    /// Top inset.
    #[cfg_attr(feature = "serde", serde(rename = "top_mm"))]
    pub top: Millimeters,
    /// Right inset.
    #[cfg_attr(feature = "serde", serde(rename = "right_mm"))]
    pub right: Millimeters,
    /// Bottom inset.
    #[cfg_attr(feature = "serde", serde(rename = "bottom_mm"))]
    pub bottom: Millimeters,
    /// Left inset.
    #[cfg_attr(feature = "serde", serde(rename = "left_mm"))]
    pub left: Millimeters,
}

impl Margins {
    /// Creates equal non-negative insets from a millimeter value.
    pub fn uniform_mm(value: f64) -> Result<Self, PaperGridError> {
        let value = Millimeters::non_negative(value)?;
        Ok(Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        })
    }
    /// Creates asymmetric non-negative insets from millimeter values.
    pub fn from_mm(top: f64, right: f64, bottom: f64, left: f64) -> Result<Self, PaperGridError> {
        Ok(Self {
            top: Millimeters::non_negative(top)?,
            right: Millimeters::non_negative(right)?,
            bottom: Millimeters::non_negative(bottom)?,
            left: Millimeters::non_negative(left)?,
        })
    }
}

/// An axis-aligned rectangle in millimeters.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct Rect {
    /// Horizontal offset from the page's left edge.
    #[cfg_attr(feature = "serde", serde(rename = "x_mm"))]
    pub x: f64,
    /// Vertical offset from the page's top edge.
    #[cfg_attr(feature = "serde", serde(rename = "y_mm"))]
    pub y: f64,
    /// Rectangle width.
    #[cfg_attr(feature = "serde", serde(rename = "width_mm"))]
    pub width: f64,
    /// Rectangle height.
    #[cfg_attr(feature = "serde", serde(rename = "height_mm"))]
    pub height: f64,
}

impl Rect {
    /// Creates a finite rectangle with non-negative dimensions.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Result<Self, PaperGridError> {
        if [x, y, width, height].iter().all(|v| v.is_finite()) && width >= 0.0 && height >= 0.0 {
            Ok(Self {
                x,
                y,
                width,
                height,
            })
        } else {
            Err(PaperGridError::InvalidMeasurement("rectangle"))
        }
    }
    /// Returns the right edge.
    pub const fn right(self) -> f64 {
        self.x + self.width
    }
    /// Returns the bottom edge.
    pub const fn bottom(self) -> f64 {
        self.y + self.height
    }
    /// Returns whether this rectangle contains `other`, allowing a tiny float tolerance.
    pub fn contains(self, other: Self) -> bool {
        let e = 1e-9;
        other.x + e >= self.x
            && other.y + e >= self.y
            && other.right() <= self.right() + e
            && other.bottom() <= self.bottom() + e
    }
}
