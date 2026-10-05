//! Deterministic, self-contained SVG rendering.

use std::fmt::Write;

use crate::{CalendarSheet, MonthRelation, error::PaperGridError};

/// Small set of layout colors, typography sizes, and line width for SVG output.
#[derive(Debug, Clone, PartialEq)]
pub struct SvgTheme {
    /// Grid stroke color.
    pub grid_stroke: String,
    /// Primary text color.
    pub text_color: String,
    /// Adjacent-month text color.
    pub adjacent_text_color: String,
    /// Page background color.
    pub page_fill: String,
    /// Font family list in SVG/CSS form.
    pub font_family: String,
    /// Grid stroke width in millimeters.
    pub stroke_width_mm: f64,
    /// Title font size in millimeters.
    pub title_font_size_mm: f64,
    /// Weekday font size in millimeters.
    pub weekday_font_size_mm: f64,
    /// Day number font size in millimeters.
    pub day_font_size_mm: f64,
}

impl Default for SvgTheme {
    fn default() -> Self {
        Self {
            grid_stroke: "#334155".into(),
            text_color: "#0f172a".into(),
            adjacent_text_color: "#64748b".into(),
            page_fill: "#ffffff".into(),
            font_family: "Arial, Helvetica, sans-serif".into(),
            stroke_width_mm: 0.25,
            title_font_size_mm: 7.0,
            weekday_font_size_mm: 3.4,
            day_font_size_mm: 4.0,
        }
    }
}

/// Renders a sheet with the default monochrome-friendly theme.
pub fn render_svg(sheet: &CalendarSheet) -> Result<String, PaperGridError> {
    render_svg_with_theme(sheet, &SvgTheme::default())
}

/// Renders a sheet with caller-supplied styling and no remote resources or scripts.
pub fn render_svg_with_theme(
    sheet: &CalendarSheet,
    theme: &SvgTheme,
) -> Result<String, PaperGridError> {
    for (value, label) in [
        (theme.stroke_width_mm, "stroke width"),
        (theme.title_font_size_mm, "title font size"),
        (theme.weekday_font_size_mm, "weekday font size"),
        (theme.day_font_size_mm, "day font size"),
    ] {
        if !value.is_finite() || value <= 0.0 {
            return Err(PaperGridError::InvalidMeasurement(label));
        }
    }
    let mut out = String::with_capacity(4096);
    writeln!(out, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}mm\" height=\"{}mm\" viewBox=\"0 0 {} {}\" role=\"img\">",
        n(sheet.page_bounds.width), n(sheet.page_bounds.height), n(sheet.page_bounds.width), n(sheet.page_bounds.height)).unwrap();
    writeln!(
        out,
        "  <rect class=\"page\" width=\"100%\" height=\"100%\" fill=\"{}\"/>",
        escape(&theme.page_fill)
    )
    .unwrap();
    writeln!(
        out,
        "  <g font-family=\"{}\" fill=\"{}\">",
        escape(&theme.font_family),
        escape(&theme.text_color)
    )
    .unwrap();
    if let (Some(title), Some(bounds)) = (&sheet.title, sheet.title_bounds) {
        writeln!(
            out,
            "    <text class=\"title\" x=\"{}\" y=\"{}\" font-size=\"{}\">{}</text>",
            n(bounds.x),
            n(bounds.y + bounds.height * 0.68),
            n(theme.title_font_size_mm),
            escape(title)
        )
        .unwrap();
    }
    for heading in &sheet.weekday_headers {
        writeln!(out, "    <text class=\"weekday\" x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"{}\">{}</text>",
            n(heading.bounds.x + heading.bounds.width / 2.0), n(heading.bounds.y + heading.bounds.height * 0.66),
            n(theme.weekday_font_size_mm), heading.weekday.name()).unwrap();
    }
    writeln!(out, "  </g>").unwrap();
    writeln!(
        out,
        "  <g class=\"calendar-grid\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\">",
        escape(&theme.grid_stroke),
        n(theme.stroke_width_mm)
    )
    .unwrap();
    for cell in &sheet.grid.cells {
        writeln!(out, "    <rect class=\"cell\" data-row=\"{}\" data-column=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
            cell.row, cell.column, n(cell.bounds.x), n(cell.bounds.y), n(cell.bounds.width), n(cell.bounds.height)).unwrap();
    }
    writeln!(out, "  </g>").unwrap();
    for cell in &sheet.grid.cells {
        if let Some(day) = cell.date.and_then(|d| d.day_number) {
            let color = if cell
                .date
                .is_some_and(|d| d.relation != MonthRelation::Current)
            {
                &theme.adjacent_text_color
            } else {
                &theme.text_color
            };
            writeln!(out, "  <text class=\"day\" data-row=\"{}\" data-column=\"{}\" x=\"{}\" y=\"{}\" fill=\"{}\" font-family=\"{}\" font-size=\"{}\">{day}</text>",
                cell.row, cell.column, n(cell.bounds.x + 1.5), n(cell.bounds.y + 5.5), escape(color), escape(&theme.font_family), n(theme.day_font_size_mm)).unwrap();
        }
    }
    if let Some(notes) = sheet.notes_bounds {
        writeln!(out, "  <rect class=\"notes-area\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"/>",
            n(notes.x), n(notes.y), n(notes.width), n(notes.height), escape(&theme.grid_stroke), n(theme.stroke_width_mm)).unwrap();
        writeln!(out, "  <text class=\"notes-label\" x=\"{}\" y=\"{}\" fill=\"{}\" font-family=\"{}\" font-size=\"{}\">Notes</text>",
            n(notes.x + 1.5), n(notes.y + 5.5), escape(&theme.text_color), escape(&theme.font_family), n(theme.weekday_font_size_mm)).unwrap();
    }
    writeln!(out, "</svg>").unwrap();
    Ok(out)
}

fn n(value: f64) -> String {
    format!("{value:.3}")
}
fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
