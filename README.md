# BetaCalendars PaperGrid

[![crates.io](https://img.shields.io/crates/v/betacalendars-papergrid.svg)](https://crates.io/crates/betacalendars-papergrid)
[![docs.rs](https://docs.rs/betacalendars-papergrid/badge.svg)](https://docs.rs/betacalendars-papergrid)
[![CI](https://github.com/mateopedersen/betacalendars-papergrid/actions/workflows/ci.yml/badge.svg)](https://github.com/mateopedersen/betacalendars-papergrid/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/betacalendars-papergrid.svg)](LICENSE-MIT)

BetaCalendars PaperGrid is a deterministic Rust library and CLI for compiling civil months and blank planner grids into physical print layouts. It handles **calendar model → grid placement → page geometry → SVG or JSON**, without coupling the calendar model to a UI or PDF stack.

## What it does

PaperGrid specializes in printable layout. It is useful in static-site generators, SVG and PDF pipelines, desktop applications, batch calendar tools, and layout QA. It does not handle appointments, timezones, or scheduling.

- Validated Gregorian civil dates and month grids with natural or six-week rows.
- Seven weekday columns with any weekday as the first column, and optional adjacent-month dates.
- A4, A5, Letter, Legal, and validated custom page dimensions, in portrait or landscape.
- Asymmetric margins, title and weekday regions, cell gaps, notes areas, and footers.
- Blank date-free grids with configurable rows and columns.
- Deterministic, self-contained SVG and structured JSON of the **calculated geometry**.
- A CLI for single months, blank sheets, and inclusive month-range batches.

## Installation

```sh
cargo add betacalendars-papergrid
```

For JSON serialization in a Rust application:

```sh
cargo add betacalendars-papergrid --features serde
```

Install the CLI with SVG and JSON output enabled:

```sh
cargo install betacalendars-papergrid --features cli
```

## Quick start

```rust
use betacalendars_papergrid::{compile_month, render::svg::render_svg, LayoutSpec, Paper, WeekStart};

let layout = LayoutSpec::builder()
    .paper(Paper::A4)
    .week_start(WeekStart::Monday)
    .fixed_six_weeks(true)
    .build()?;
let sheet = compile_month(2027, 1, &layout)?;
let svg = render_svg(&sheet)?;
assert_eq!(sheet.grid.cells.len(), 42);
# Ok::<(), Box<dyn std::error::Error>>(())
```

`CivilDate::new(year, month, day)` validates every date. The supported year range is CE 1 through 9999. Dates contain no time or timezone data. Geometry is measured in millimeters and uses finite `f64` calculations; the last cell in each row/column is fitted to the exact grid edge to avoid accumulated drift.

## Month grids and page layout

`MonthModel` provides validated Gregorian month facts and date placements separately from rendering. `LayoutSpec::builder()` selects paper, orientation, margins, weekday start, row mode, adjacent-day visibility, cell gap, and optional notes/footer heights. `compile_month(year, month, &layout)` returns the complete `CalendarSheet`: physical page and printable bounds, title and weekday areas, row-major cell rectangles, optional notes, and month metadata.

The built-in paper dimensions are A4 (210 × 297 mm), A5 (148 × 210 mm), Letter (215.9 × 279.4 mm), and Legal (215.9 × 355.6 mm). `PaperSize::custom_mm(width, height)` supports other positive finite dimensions. Layout building rejects impossible margins, negative or non-finite measurements, and pages without positive grid-cell area.

## Blank planner sheets

`BlankCalendarSpec::builder()` compiles an undated grid independently of a month. It defaults to six rows and seven weekday columns; row and column counts may be configured from one through 31. Weekday headers require seven columns. Optional title and notes regions share the same physical layout rules as dated sheets.

For a human-facing blank layout to compare with generated output, see the [Beta Calendars blank calendar](https://www.betacalendars.com/blank-calendar).

## SVG and JSON

SVG is enabled by default. `render::svg::render_svg` produces byte-for-byte stable output for identical input and theme, with page dimensions, a `viewBox`, semantic groups, no scripts, no remote resources, and escaped text. `SvgTheme` provides small, explicit controls for colors, font family, and sizes.

Enable the `serde` feature to serialize the resulting `CalendarSheet` as JSON. `render::json::render_json_pretty` includes oriented paper dimensions, page and printable bounds, month or blank metadata, rows/columns, and each cell's computed rectangle and optional date placement.

## CLI

```sh
# One month
papergrid month --year 2027 --month 1 --paper a4 --orientation portrait \
  --week-start monday --fixed-weeks --output january.svg

# A date-free Letter planner sheet
papergrid blank --paper letter --orientation landscape --rows 6 \
  --weekday-header true --notes-height-mm 28 --output blank.json --format json

# Inclusive range: creates 2026-11.svg through 2027-02.svg
papergrid range --from 2026-11 --to 2027-02 --paper a4 \
  --format svg --output ./winter-calendar/
```

`month` and `blank` support `--format svg|json`. `range` writes one deterministically named file per month into the output directory. Reversed ranges, invalid months, and impossible layouts return an error and a non-zero exit code.

## Reference fixtures: the 2026–2027 year boundary

The repository's four-month examples and tests cross December into January. Their weekday positions are checked against an independent Gregorian calendar calculation:

- [November 2026 reference calendar](https://www.betacalendars.com/november-calendar.html)
- [December 2026 reference calendar](https://www.betacalendars.com/december-calendar.html)
- [January 2027 reference calendar](https://www.betacalendars.com/january-calendar.html)
- [February 2027 reference calendar](https://www.betacalendars.com/february-calendar.html)

## Determinism and feature flags

The library reads no clock, locale, timezone, environment variables, or network resources. The same specification yields the same geometry and renderer output across machines.

| Feature | Effect |
| --- | --- |
| `svg` (default) | SVG renderer |
| `serde` | `Serialize` implementations and JSON renderer |
| `cli` | `papergrid` binary, JSON, and SVG |

## Rust version and documentation

PaperGrid uses Rust 2024 Edition and declares Rust 1.85 as its minimum supported version (MSRV). Full API documentation is published on [docs.rs](https://docs.rs/betacalendars-papergrid).

## Project

PaperGrid is maintained as part of the [Beta Calendars](https://www.betacalendars.com/) calendar and printable-layout project. The crate itself is an independent, offline-capable Rust package.

## License

Licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
