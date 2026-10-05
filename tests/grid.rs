use betacalendars_papergrid::{GridMode, GridOptions, MonthModel, MonthRelation, WeekStart};

#[test]
fn fixed_grid_has_42_cells_and_sequential_current_days() {
    let month = MonthModel::new(2027, 1).unwrap();
    let cells = month.grid(GridOptions {
        week_start: WeekStart::Monday,
        mode: GridMode::FixedSixWeeks,
        adjacent_days: true,
    });
    assert_eq!(cells.len(), 42);
    let current: Vec<_> = cells
        .iter()
        .filter(|cell| cell.relation == MonthRelation::Current)
        .collect();
    assert_eq!(current.len(), 31);
    assert!(
        current
            .iter()
            .enumerate()
            .all(|(index, cell)| cell.day_number == Some(index as u8 + 1))
    );
    assert!(current.iter().all(|cell| cell.date.is_some()));
}

#[test]
fn natural_grid_uses_four_through_six_complete_weeks() {
    for month in 1..=12 {
        let model = MonthModel::new(2027, month).unwrap();
        let grid = model.grid(GridOptions::default());
        assert!((4..=6).contains(&(grid.len() / 7)));
        assert_eq!(grid.len() % 7, 0);
    }
}

#[test]
fn hidden_adjacent_dates_keep_their_relation_but_not_a_label() {
    let model = MonthModel::new(2026, 11).unwrap();
    let grid = model.grid(GridOptions {
        week_start: WeekStart::Monday,
        mode: GridMode::FixedSixWeeks,
        adjacent_days: false,
    });
    assert!(
        grid.iter()
            .filter(|cell| cell.relation != MonthRelation::Current)
            .all(|cell| cell.date.is_none() && cell.day_number.is_none())
    );
    assert!(
        grid.iter()
            .filter(|cell| cell.relation == MonthRelation::Current)
            .all(|cell| cell.date.is_some())
    );
}

#[test]
fn each_first_weekday_places_the_first_day_in_column_zero() {
    let month = MonthModel::new(2026, 11).unwrap();
    for week_start in [
        WeekStart::Monday,
        WeekStart::Tuesday,
        WeekStart::Wednesday,
        WeekStart::Thursday,
        WeekStart::Friday,
        WeekStart::Saturday,
        WeekStart::Sunday,
    ] {
        let grid = month.grid(GridOptions {
            week_start,
            mode: GridMode::Natural,
            adjacent_days: true,
        });
        let first = grid
            .iter()
            .find(|cell| cell.day_number == Some(1) && cell.relation == MonthRelation::Current)
            .unwrap();
        assert_eq!(first.column, (6 + 7 - week_start.index()) as usize % 7);
        assert_eq!(first.weekday, betacalendars_papergrid::Weekday::Sunday);
    }
}
