use std::{
    fs,
    path::{Path, PathBuf},
};

use betacalendars_papergrid::{
    GridMode, LayoutSpec, Orientation, Paper, PaperGridError, WeekStart,
    blank::BlankCalendarSpec,
    compile_blank, compile_month,
    render::{json::render_json_pretty, svg::render_svg},
};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "papergrid",
    version,
    about = "Compile printable calendar sheets to deterministic SVG or JSON"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Compile one dated Gregorian month.
    Month {
        #[arg(long)]
        year: u16,
        #[arg(long)]
        month: u8,
        #[arg(long, value_enum, default_value_t = PaperArg::A4)]
        paper: PaperArg,
        #[arg(long, value_enum, default_value_t = OrientationArg::Portrait)]
        orientation: OrientationArg,
        #[arg(long, value_enum, default_value_t = WeekStartArg::Monday)]
        week_start: WeekStartArg,
        #[arg(long)]
        fixed_weeks: bool,
        #[arg(long)]
        hide_adjacent_days: bool,
        #[arg(long)]
        notes_height_mm: Option<f64>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Svg)]
        format: OutputFormat,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Compile an undated planner grid.
    Blank {
        #[arg(long, value_enum, default_value_t = PaperArg::A4)]
        paper: PaperArg,
        #[arg(long, value_enum, default_value_t = OrientationArg::Portrait)]
        orientation: OrientationArg,
        #[arg(long, default_value_t = 6)]
        rows: usize,
        #[arg(long, default_value_t = 7)]
        columns: usize,
        #[arg(long, value_enum, default_value_t = WeekStartArg::Monday)]
        week_start: WeekStartArg,
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        weekday_header: bool,
        #[arg(long, default_value = "Blank calendar")]
        title: String,
        #[arg(long)]
        notes_height_mm: Option<f64>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Svg)]
        format: OutputFormat,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Compile an inclusive month range to a directory of files.
    Range {
        #[arg(long, value_parser = parse_month)]
        from: (u16, u8),
        #[arg(long, value_parser = parse_month)]
        to: (u16, u8),
        #[arg(long, value_enum, default_value_t = PaperArg::A4)]
        paper: PaperArg,
        #[arg(long, value_enum, default_value_t = OrientationArg::Portrait)]
        orientation: OrientationArg,
        #[arg(long, value_enum, default_value_t = WeekStartArg::Monday)]
        week_start: WeekStartArg,
        #[arg(long)]
        fixed_weeks: bool,
        #[arg(long)]
        notes_height_mm: Option<f64>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Svg)]
        format: OutputFormat,
        #[arg(long, default_value = "calendar-range")]
        output: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum PaperArg {
    A4,
    Letter,
    A5,
    Legal,
}
impl From<PaperArg> for Paper {
    fn from(v: PaperArg) -> Self {
        match v {
            PaperArg::A4 => Self::A4,
            PaperArg::Letter => Self::Letter,
            PaperArg::A5 => Self::A5,
            PaperArg::Legal => Self::Legal,
        }
    }
}
#[derive(Debug, Clone, Copy, ValueEnum)]
enum OrientationArg {
    Portrait,
    Landscape,
}
impl From<OrientationArg> for Orientation {
    fn from(v: OrientationArg) -> Self {
        match v {
            OrientationArg::Portrait => Self::Portrait,
            OrientationArg::Landscape => Self::Landscape,
        }
    }
}
#[derive(Debug, Clone, Copy, ValueEnum)]
enum WeekStartArg {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl From<WeekStartArg> for WeekStart {
    fn from(v: WeekStartArg) -> Self {
        match v {
            WeekStartArg::Monday => Self::Monday,
            WeekStartArg::Tuesday => Self::Tuesday,
            WeekStartArg::Wednesday => Self::Wednesday,
            WeekStartArg::Thursday => Self::Thursday,
            WeekStartArg::Friday => Self::Friday,
            WeekStartArg::Saturday => Self::Saturday,
            WeekStartArg::Sunday => Self::Sunday,
        }
    }
}
#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Svg,
    Json,
}

fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("papergrid: {error}");
        std::process::exit(2);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Month {
            year,
            month,
            paper,
            orientation,
            week_start,
            fixed_weeks,
            hide_adjacent_days,
            notes_height_mm,
            format,
            output,
        } => {
            let layout = base_layout(paper, orientation, week_start, fixed_weeks, notes_height_mm)?
                .adjacent_days(!hide_adjacent_days)
                .build()?;
            let sheet = compile_month(year, month, &layout)?;
            let output = output.unwrap_or_else(|| default_output("calendar", format));
            write_output(&output, &sheet, format)?;
        }
        Command::Blank {
            paper,
            orientation,
            rows,
            columns,
            week_start,
            weekday_header,
            title,
            notes_height_mm,
            format,
            output,
        } => {
            let blank = BlankCalendarSpec::builder()
                .paper(paper.into())
                .orientation(orientation.into())
                .rows(rows)
                .columns(columns)
                .week_start(week_start.into())
                .weekday_header(weekday_header)
                .title(title)
                .notes_height_mm(notes_height_mm)
                .build()?;
            let sheet = compile_blank(&blank)?;
            let output = output.unwrap_or_else(|| default_output("blank-calendar", format));
            write_output(&output, &sheet, format)?;
        }
        Command::Range {
            from,
            to,
            paper,
            orientation,
            week_start,
            fixed_weeks,
            notes_height_mm,
            format,
            output,
        } => {
            if month_index(from) > month_index(to) {
                return Err(PaperGridError::InvalidDateRange.into());
            }
            fs::create_dir_all(&output)?;
            let layout = base_layout(paper, orientation, week_start, fixed_weeks, notes_height_mm)?
                .build()?;
            let mut index = month_index(from);
            let end = month_index(to);
            while index <= end {
                let year = (index / 12 + 1) as u16;
                let month = (index % 12 + 1) as u8;
                let sheet = compile_month(year, month, &layout)?;
                let extension = match format {
                    OutputFormat::Svg => "svg",
                    OutputFormat::Json => "json",
                };
                let path = output.join(format!("{year:04}-{month:02}.{extension}"));
                write_output(&path, &sheet, format)?;
                index += 1;
            }
        }
    }
    Ok(())
}

fn base_layout(
    paper: PaperArg,
    orientation: OrientationArg,
    week_start: WeekStartArg,
    fixed: bool,
    notes: Option<f64>,
) -> Result<betacalendars_papergrid::LayoutSpecBuilder, PaperGridError> {
    Ok(LayoutSpec::builder()
        .paper(paper.into())
        .orientation(orientation.into())
        .week_start(week_start.into())
        .grid_mode(if fixed {
            GridMode::FixedSixWeeks
        } else {
            GridMode::Natural
        })
        .notes_height_mm(notes))
}

fn write_output(
    path: &Path,
    sheet: &betacalendars_papergrid::CalendarSheet,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = match format {
        OutputFormat::Svg => render_svg(sheet)?,
        OutputFormat::Json => render_json_pretty(sheet)?,
    };
    fs::write(path, output)?;
    Ok(())
}

fn default_output(stem: &str, format: OutputFormat) -> PathBuf {
    let extension = match format {
        OutputFormat::Svg => "svg",
        OutputFormat::Json => "json",
    };
    PathBuf::from(format!("{stem}.{extension}"))
}

fn parse_month(input: &str) -> Result<(u16, u8), String> {
    let (year, month) = input
        .split_once('-')
        .ok_or_else(|| "expected YYYY-MM".to_owned())?;
    let year: u16 = year
        .parse()
        .map_err(|_| "year must be an integer".to_owned())?;
    let month: u8 = month
        .parse()
        .map_err(|_| "month must be an integer".to_owned())?;
    betacalendars_papergrid::MonthModel::new(year, month).map_err(|e| e.to_string())?;
    Ok((year, month))
}

const fn month_index((year, month): (u16, u8)) -> u32 {
    (year as u32 - 1) * 12 + month as u32 - 1
}
