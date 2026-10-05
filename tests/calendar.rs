use betacalendars_papergrid::{
    CivilDate, MonthModel, PaperGridError, Weekday,
    calendar::{days_in_month, is_leap_year},
};

#[test]
fn date_constructor_rejects_out_of_range_values() {
    assert!(matches!(
        CivilDate::new(0, 1, 1),
        Err(PaperGridError::InvalidYear(0))
    ));
    assert!(matches!(
        CivilDate::new(10_000, 1, 1),
        Err(PaperGridError::InvalidYear(10_000))
    ));
    assert!(matches!(
        CivilDate::new(2027, 0, 1),
        Err(PaperGridError::InvalidMonth(0))
    ));
    assert!(matches!(
        CivilDate::new(2027, 13, 1),
        Err(PaperGridError::InvalidMonth(13))
    ));
    assert!(CivilDate::new(2027, 2, 29).is_err());
    assert!(CivilDate::new(2027, 4, 31).is_err());
}

#[test]
fn leap_year_rule_and_month_lengths_are_gregorian() {
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2000));
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2027));
    assert_eq!(days_in_month(2027, 2), 28);
    assert_eq!(days_in_month(2028, 2), 29);
    assert_eq!(days_in_month(2027, 13), 0);
}

#[test]
fn month_model_iterates_valid_dates_in_order() {
    let month = MonthModel::new(2027, 2).unwrap();
    let days: Vec<_> = month.days().collect();
    assert_eq!(days.len(), 28);
    assert_eq!(days.first().unwrap().day(), 1);
    assert_eq!(days.last().unwrap().day(), 28);
    assert!(
        days.windows(2)
            .all(|pair| pair[1].day() == pair[0].day() + 1)
    );
    assert_eq!(
        CivilDate::new(2027, 2, 1).unwrap().weekday(),
        Weekday::Monday
    );
}
