//! JSON serialization of calculated sheet geometry.

use crate::CalendarSheet;

/// Serializes calculated page geometry as compact JSON.
pub fn render_json(sheet: &CalendarSheet) -> Result<String, serde_json::Error> {
    serde_json::to_string(sheet)
}

/// Serializes calculated page geometry as indented JSON.
pub fn render_json_pretty(sheet: &CalendarSheet) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(sheet)
}
