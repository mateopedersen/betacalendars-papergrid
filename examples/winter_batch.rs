use std::{fs, path::PathBuf};

use betacalendars_papergrid::{
    LayoutSpec, Paper, WeekStart, compile_month, render::svg::render_svg,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from("winter-calendar");
    fs::create_dir_all(&output)?;
    let layout = LayoutSpec::builder()
        .paper(Paper::A4)
        .week_start(WeekStart::Monday)
        .fixed_six_weeks(true)
        .build()?;
    for (year, month) in [(2026, 11), (2026, 12), (2027, 1), (2027, 2)] {
        let path = output.join(format!("{year:04}-{month:02}.svg"));
        fs::write(&path, render_svg(&compile_month(year, month, &layout)?)?)?;
        println!("wrote {}", path.display());
    }
    Ok(())
}
