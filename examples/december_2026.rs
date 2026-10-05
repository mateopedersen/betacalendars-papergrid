use betacalendars_papergrid::{
    LayoutSpec, Paper, WeekStart, compile_month, render::svg::render_svg,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layout = LayoutSpec::builder()
        .paper(Paper::A4)
        .week_start(WeekStart::Monday)
        .fixed_six_weeks(true)
        .build()?;
    print!("{}", render_svg(&compile_month(2026, 12, &layout)?)?);
    Ok(())
}
