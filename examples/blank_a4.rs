use betacalendars_papergrid::{
    BlankCalendarSpec, Orientation, Paper, compile_blank, render::svg::render_svg,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let blank = BlankCalendarSpec::builder()
        .paper(Paper::A4)
        .orientation(Orientation::Portrait)
        .rows(6)
        .weekday_header(true)
        .notes_height_mm(Some(30.0))
        .build()?;
    print!("{}", render_svg(&compile_blank(&blank)?)?);
    Ok(())
}
