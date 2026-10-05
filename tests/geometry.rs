use betacalendars_papergrid::{
    LayoutSpec, Orientation, Paper, PaperGridError, PaperSize, Rect, compile_month,
};

#[test]
fn built_in_pages_and_orientations_keep_all_cells_inside_page() {
    for paper in [Paper::A4, Paper::A5, Paper::Letter, Paper::Legal] {
        for orientation in [Orientation::Portrait, Orientation::Landscape] {
            let layout = LayoutSpec::builder()
                .paper(paper)
                .orientation(orientation)
                .fixed_six_weeks(true)
                .build()
                .unwrap();
            let sheet = compile_month(2027, 1, &layout).unwrap();
            assert_eq!(sheet.grid.cells.len(), 42);
            assert!(sheet.page_bounds.contains(sheet.printable_bounds));
            assert!(
                sheet
                    .grid
                    .cells
                    .iter()
                    .all(|cell| sheet.grid.bounds.contains(cell.bounds))
            );
            assert!(
                sheet
                    .grid
                    .cells
                    .iter()
                    .all(|cell| sheet.page_bounds.contains(cell.bounds))
            );
            assert!(
                sheet
                    .grid
                    .cells
                    .iter()
                    .all(|cell| cell.bounds.width > 0.0 && cell.bounds.height > 0.0)
            );
        }
    }
}

#[test]
fn notes_footer_and_asymmetric_margins_are_inside_printable_bounds() {
    let layout = LayoutSpec::builder()
        .paper(Paper::Letter)
        .orientation(Orientation::Landscape)
        .margins_mm(8.0, 14.0, 10.0, 12.0)
        .notes_height_mm(Some(26.0))
        .footer_height_mm(9.0)
        .build()
        .unwrap();
    let sheet = compile_month(2026, 12, &layout).unwrap();
    let notes = sheet.notes_bounds.unwrap();
    let footer = sheet.footer_bounds.unwrap();
    assert!(sheet.printable_bounds.contains(notes));
    assert!(sheet.printable_bounds.contains(footer));
    assert!(sheet.printable_bounds.contains(sheet.grid.bounds));
    assert!((notes.bottom() - footer.y).abs() < 1e-9);
    assert!((sheet.grid.bounds.bottom() - notes.y).abs() < 1e-9);
}

#[test]
fn impossible_margins_and_tiny_custom_paper_are_rejected() {
    assert!(matches!(
        LayoutSpec::builder()
            .margins_mm(160.0, 160.0, 0.0, 60.0)
            .build(),
        Err(PaperGridError::InvalidMargins)
    ));
    let tiny = PaperSize::custom_mm(80.0, 100.0).unwrap();
    let layout = LayoutSpec::builder()
        .paper_size(tiny)
        .uniform_margins_mm(5.0)
        .cell_gap_mm(12.0)
        .build()
        .unwrap();
    assert_eq!(
        compile_month(2027, 1, &layout).unwrap_err(),
        PaperGridError::InsufficientPrintableArea
    );
}

#[test]
fn last_column_and_row_terminate_at_the_exact_grid_edges() {
    let layout = LayoutSpec::builder()
        .cell_gap_mm(0.7)
        .fixed_six_weeks(true)
        .build()
        .unwrap();
    let sheet = compile_month(2027, 1, &layout).unwrap();
    let last = sheet.grid.cells.last().unwrap().bounds;
    assert!((last.right() - sheet.grid.bounds.right()).abs() < 1e-9);
    assert!((last.bottom() - sheet.grid.bounds.bottom()).abs() < 1e-9);
    let _rect = Rect::new(0.0, 0.0, 1.0, 1.0).unwrap();
}
