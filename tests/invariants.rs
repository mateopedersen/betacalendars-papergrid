use betacalendars_papergrid::{
    GridMode, GridOptions, LayoutSpec, MonthModel, MonthRelation, WeekStart, compile_month,
};

#[test]
fn four_hundred_year_cycle_preserves_month_and_grid_invariants() {
    for year in 2000..2400 {
        for month in 1..=12 {
            let model = MonthModel::new(year, month).unwrap();
            let grid = model.grid(GridOptions {
                week_start: WeekStart::Sunday,
                mode: GridMode::FixedSixWeeks,
                adjacent_days: true,
            });
            assert_eq!(grid.len(), 42);
            let current: Vec<_> = grid
                .iter()
                .filter(|cell| cell.relation == MonthRelation::Current)
                .collect();
            assert_eq!(current.len(), usize::from(model.day_count()));
            for (day, cell) in current.iter().enumerate() {
                assert_eq!(cell.day_number, Some(day as u8 + 1));
                assert_eq!(
                    cell.row * 7 + cell.column,
                    grid.iter()
                        .position(|item| std::ptr::eq(item, *cell))
                        .unwrap()
                );
            }
            let sheet = compile_month(
                year,
                month,
                &LayoutSpec::builder().fixed_six_weeks(true).build().unwrap(),
            )
            .unwrap();
            assert!(
                sheet
                    .grid
                    .cells
                    .iter()
                    .all(|cell| sheet.grid.bounds.contains(cell.bounds))
            );
        }
    }
}
