use betacalendars_papergrid::{
    BlankCalendarSpec, Orientation, Paper, PaperGridError, PaperSize, compile_blank,
};

#[test]
fn blank_a4_and_letter_support_both_orientations_and_notes() {
    for paper in [Paper::A4, Paper::Letter] {
        for orientation in [Orientation::Portrait, Orientation::Landscape] {
            let spec = BlankCalendarSpec::builder()
                .paper(paper)
                .orientation(orientation)
                .rows(6)
                .weekday_header(true)
                .notes_height_mm(Some(24.0))
                .build()
                .unwrap();
            let sheet = compile_blank(&spec).unwrap();
            assert_eq!(
                (sheet.grid.rows, sheet.grid.columns, sheet.grid.cells.len()),
                (6, 7, 42)
            );
            assert_eq!(sheet.weekday_headers.len(), 7);
            assert!(sheet.notes_bounds.is_some());
            assert!(sheet.grid.cells.iter().all(|cell| cell.date.is_none()
                && cell.bounds.width > 0.0
                && cell.bounds.height > 0.0));
        }
    }
}

#[test]
fn blank_grids_support_five_rows_and_custom_paper_sizes() {
    let custom = PaperSize::custom_mm(180.0, 260.0).unwrap();
    let spec = BlankCalendarSpec::builder()
        .paper_size(custom)
        .rows(5)
        .columns(7)
        .weekday_header(false)
        .margins_mm(14.0, 8.0, 12.0, 8.0)
        .notes_height_mm(None)
        .build()
        .unwrap();
    let sheet = compile_blank(&spec).unwrap();
    assert_eq!(sheet.grid.cells.len(), 35);
    assert!(sheet.weekday_headers.is_empty());
    assert!(sheet.notes_bounds.is_none());
}

#[test]
fn blank_grid_dimensions_and_header_requirements_are_checked() {
    assert!(matches!(
        BlankCalendarSpec::builder().rows(0).build(),
        Err(PaperGridError::InvalidGridDimensions { .. })
    ));
    assert!(matches!(
        BlankCalendarSpec::builder().columns(8).build(),
        Err(PaperGridError::InvalidGridDimensions { .. })
    ));
    let custom_columns = BlankCalendarSpec::builder()
        .columns(5)
        .weekday_header(true)
        .build();
    assert!(matches!(
        custom_columns,
        Err(PaperGridError::InvalidGridDimensions { .. })
    ));
}
