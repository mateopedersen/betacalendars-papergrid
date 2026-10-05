#[cfg(any(feature = "svg", feature = "serde"))]
use betacalendars_papergrid::{
    BlankCalendarSpec, LayoutSpec, compile_blank, compile_month, render,
};

#[cfg(feature = "svg")]
#[test]
fn svg_is_deterministic_escaped_and_self_contained() {
    let sheet = compile_blank(
        &BlankCalendarSpec::builder()
            .title("Plan <A&B>")
            .build()
            .unwrap(),
    )
    .unwrap();
    let first = render::svg::render_svg(&sheet).unwrap();
    let second = render::svg::render_svg(&sheet).unwrap();
    assert_eq!(first, second);
    assert!(first.starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\""));
    assert!(first.contains("Plan &lt;A&amp;B&gt;"));
    assert!(!first.contains("<script"));
    assert!(!first.contains("https://"));
}

#[cfg(feature = "svg")]
#[test]
fn calendar_svg_contains_current_dates_and_weekday_heading() {
    let sheet = compile_month(2026, 11, &LayoutSpec::builder().build().unwrap()).unwrap();
    let svg = render::svg::render_svg(&sheet).unwrap();
    assert!(svg.contains("November 2026"));
    assert!(svg.contains("Sunday"));
    assert!(svg.contains("class=\"day\""));
}

#[cfg(feature = "serde")]
#[test]
fn json_exports_calculated_geometry_not_only_configuration() {
    let sheet = compile_month(
        2027,
        1,
        &LayoutSpec::builder().fixed_six_weeks(true).build().unwrap(),
    )
    .unwrap();
    let json = render::json::render_json_pretty(&sheet).unwrap();
    assert!(json.contains("\"width_mm\": 210.0"));
    assert!(json.contains("\"rows\": 6"));
    assert!(json.contains("\"columns\": 7"));
    assert!(json.contains("\"bounds\""));
    assert!(json.contains("\"day_number\": 1"));
}
