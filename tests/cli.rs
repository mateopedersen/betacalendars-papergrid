#![cfg(feature = "cli")]

use std::{fs, process::Command};

fn papergrid() -> Command {
    Command::new(env!("CARGO_BIN_EXE_papergrid"))
}

#[test]
fn range_cli_writes_four_inclusive_month_outputs_independent_of_timezone() {
    let dir = std::env::temp_dir().join(format!("papergrid-cli-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let run = |tz: &str| {
        papergrid()
            .args(["range", "--from", "2026-11", "--to", "2027-02", "--output"])
            .arg(&dir)
            .env("TZ", tz)
            .output()
            .unwrap()
    };
    assert!(run("UTC").status.success());
    let names = ["2026-11.svg", "2026-12.svg", "2027-01.svg", "2027-02.svg"];
    let first: Vec<_> = names
        .iter()
        .map(|name| fs::read(dir.join(name)).unwrap())
        .collect();
    assert!(run("Pacific/Auckland").status.success());
    for (index, name) in names.iter().enumerate() {
        assert_eq!(first[index], fs::read(dir.join(name)).unwrap());
    }
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn cli_rejects_reversed_month_ranges() {
    let output = papergrid()
        .args(["range", "--from", "2027-02", "--to", "2026-11"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn month_json_format_uses_json_default_filename_and_measured_units() {
    let dir = std::env::temp_dir().join(format!("papergrid-json-cli-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let result = papergrid()
        .args([
            "month", "--year", "2027", "--month", "1", "--format", "json",
        ])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let json = fs::read_to_string(dir.join("calendar.json")).unwrap();
    assert!(json.contains("\"width_mm\": 210.0"));
    assert!(json.contains("\"grid\""));
    fs::remove_dir_all(dir).unwrap();
}
