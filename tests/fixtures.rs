use betacalendars_papergrid::{CivilDate, MonthModel, Weekday, calendar::days_in_month};

#[test]
fn year_boundary_reference_months_have_independently_checked_weekdays() {
    // Checked independently using Python's stdlib Gregorian datetime/calendar implementation.
    let cases = [
        (2026, 11, 30, Weekday::Sunday),
        (2026, 12, 31, Weekday::Tuesday),
        (2027, 1, 31, Weekday::Friday),
        (2027, 2, 28, Weekday::Monday),
    ];
    for (year, month, count, weekday) in cases {
        assert_eq!(days_in_month(year, month), count);
        assert_eq!(CivilDate::new(year, month, 1).unwrap().weekday(), weekday);
        assert_eq!(
            MonthModel::new(year, month).unwrap().days().count(),
            usize::from(count)
        );
    }
}
