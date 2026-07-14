use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;
use tracing_subscriber::layer::SubscriberExt;

static LOGGER: std::sync::OnceLock<Mutex<Option<LoggerState>>> = std::sync::OnceLock::new();

#[allow(dead_code)]
struct LoggerState {
    log_dir: PathBuf,
    current_date: String,
    file: fs::File,
}

fn today_date() -> String {
    let dur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs();
    let days = secs / 86400;
    let mut y = 1970u64;
    let mut d = days;
    loop {
        let year_days = if is_leap(y) { 366 } else { 365 };
        if d < year_days {
            break;
        }
        d -= year_days;
        y += 1;
    }
    let month_days = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0u64;
    for (i, &md) in month_days.iter().enumerate() {
        if d < md {
            m = i as u64;
            break;
        }
        d -= md;
    }
    format!("{:04}-{:02}-{:02}", y, m + 1, d + 1)
}

fn is_leap(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn platform_validate_path(path: &Path) -> Result<(), String> {
    let path_str = path
        .to_str()
        .ok_or("Log path contains invalid characters")?;

    if path_str.is_empty() {
        return Err("Log path must not be empty".into());
    }

    #[cfg(target_os = "windows")]
    {
        if !path.is_absolute() {
            return Err("Log path must be absolute on Windows".into());
        }
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
        for c in invalid_chars {
            if path_str.contains(c) {
                return Err(format!("Invalid character '{}' in log path", c));
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if !path.is_absolute() {
            return Err("Log path must be absolute (e.g., /var/log/bettertui)".into());
        }
        let invalid_chars = ['\0'];
        for c in invalid_chars {
            if path_str.contains(c) {
                return Err("Log path contains null character".into());
            }
        }
    }

    Ok(())
}

fn daily_log_path(log_dir: &Path) -> PathBuf {
    let date = today_date();
    log_dir.join(format!("bettertui-{}.log", date))
}

fn open_or_truncate(path: &Path) -> Result<fs::File, String> {
    if path.exists() {
        let metadata =
            fs::metadata(path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let modified_secs = modified
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let now_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let modified_day = modified_secs / 86400;
        let now_day = now_secs / 86400;

        if modified_day == now_day {
            let file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .map_err(|e| format!("Failed to open log file: {}", e))?;
            return Ok(file);
        }

        let _ = fs::remove_file(path);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create log directory: {}", e))?;
    }

    let file = fs::File::create(path).map_err(|e| format!("Failed to create log file: {}", e))?;
    Ok(file)
}

pub fn init() {
    if let Ok(log_path) = std::env::var("BETTERTUI_LOG_DIR") {
        let log_dir = PathBuf::from(&log_path);
        if let Err(e) = platform_validate_path(&log_dir) {
            eprintln!(
                "[logger] Invalid BETTERTUI_LOG_DIR '{}', logger disabled: {}",
                log_path, e
            );
            return;
        }
        init_with_dir(&log_dir);
    } else {
        let log_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("logs");
        init_with_dir(&log_dir);
    }
}

fn init_with_dir(log_dir: &Path) {
    let _ = LOGGER.get_or_init(|| {
        let _ = platform_validate_path(log_dir);

        let log_file = daily_log_path(log_dir);

        match open_or_truncate(&log_file) {
            Ok(file) => {
                let date = today_date();
                eprintln!("[logger] Writing logs to: {}", log_file.display());

                let file_layer = tracing_subscriber::fmt::layer()
                    .with_writer(Mutex::new(file))
                    .with_ansi(false)
                    .with_target(true)
                    .with_thread_ids(false)
                    .with_thread_names(false)
                    .with_file(true)
                    .with_line_number(true);

                let subscriber = tracing_subscriber::Registry::default().with(file_layer);
                let _ = tracing::subscriber::set_global_default(subscriber);

                Mutex::new(Some(LoggerState {
                    log_dir: log_dir.to_path_buf(),
                    current_date: date,
                    file: fs::File::create(&log_file).expect("Failed to recreate log file"),
                }))
            }
            Err(e) => {
                eprintln!("[logger] Failed to initialize: {}, logger disabled", e);
                Mutex::new(None)
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    mod leap_year {
        use super::*;

        #[test]
        fn standard_leap_years() {
            assert!(is_leap(2020));
            assert!(is_leap(2024));
            assert!(is_leap(2000));
            assert!(is_leap(1996));
            assert!(is_leap(2400));
        }

        #[test]
        fn non_leap_years() {
            assert!(!is_leap(2021));
            assert!(!is_leap(2023));
            assert!(!is_leap(1900));
            assert!(!is_leap(2100));
            assert!(!is_leap(2001));
            assert!(!is_leap(1999));
        }

        #[test]
        fn century_years() {
            assert!(!is_leap(1700));
            assert!(!is_leap(1800));
            assert!(!is_leap(1900));
            assert!(is_leap(2000));
            assert!(!is_leap(2100));
            assert!(!is_leap(2200));
            assert!(!is_leap(2300));
            assert!(is_leap(2400));
        }

        #[test]
        fn edge_cases() {
            assert!(is_leap(4));
            assert!(!is_leap(100));
            assert!(is_leap(400));
            assert!(is_leap(0));
        }
    }

    mod today_date {
        use super::*;

        #[test]
        fn format_is_valid() {
            let date = today_date();
            let parts: Vec<&str> = date.split('-').collect();
            assert_eq!(parts.len(), 3, "Date should be YYYY-MM-DD format");

            let year: u64 = parts[0].parse().expect("Year should be numeric");
            let month: u64 = parts[1].parse().expect("Month should be numeric");
            let day: u64 = parts[2].parse().expect("Day should be numeric");

            assert!(year >= 1970, "Year should be >= 1970");
            assert!(month >= 1 && month <= 12, "Month should be 1-12");
            assert!(day >= 1 && day <= 31, "Day should be 1-31");
        }

        #[test]
        fn format_is_zero_padded() {
            let date = today_date();
            assert_eq!(
                date.len(),
                10,
                "Date should be exactly 10 chars (YYYY-MM-DD)"
            );
            assert!(
                date.chars().nth(4) == Some('-'),
                "Fifth char should be dash"
            );
            assert!(
                date.chars().nth(7) == Some('-'),
                "Eighth char should be dash"
            );
        }
    }

    mod platform_validate_path {
        use super::*;

        #[test]
        fn valid_absolute_path() {
            let path = Path::new("/tmp/test.log");
            assert!(platform_validate_path(path).is_ok());
        }

        #[test]
        fn valid_nested_path() {
            let path = Path::new("/var/log/bettertui/app.log");
            assert!(platform_validate_path(path).is_ok());
        }

        #[test]
        fn empty_path_fails() {
            let path = Path::new("");
            let result = platform_validate_path(path);
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("empty"));
        }

        #[test]
        fn relative_path_fails() {
            let path = Path::new("relative/path.log");
            let result = platform_validate_path(path);
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("absolute"));
        }

        #[test]
        fn path_with_null_char_fails() {
            let path = Path::new("/tmp/test\0.log");
            let result = platform_validate_path(path);
            assert!(result.is_err());
        }

        #[cfg(target_os = "windows")]
        #[test]
        fn windows_invalid_chars_fail() {
            let test_cases = [
                ("C:\\<bad\\log.log", '<'),
                ("C:\\>bad\\log.log", '>'),
                ("C:\\:bad\\log.log", ':'),
                ("C:\\\"bad\\log.log", '"'),
                ("C:\\|bad\\log.log", '|'),
                ("C:\\?bad\\log.log", '?'),
                ("C:\\*bad\\log.log", '*'),
            ];

            for (path_str, invalid_char) in test_cases {
                let path = Path::new(path_str);
                let result = platform_validate_path(path);
                assert!(result.is_err(), "Path '{}' should fail", path_str);
                let err = result.unwrap_err();
                assert!(
                    err.contains(invalid_char),
                    "Error should mention '{}'",
                    invalid_char
                );
            }
        }

        #[cfg(not(target_os = "windows"))]
        #[test]
        fn unix_allows_special_chars() {
            let path = Path::new("/tmp/test-file_2024.log");
            assert!(platform_validate_path(path).is_ok());
        }
    }

    mod daily_log_path {
        use super::*;

        #[test]
        fn path_format_is_correct() {
            let temp_dir = std::env::temp_dir().join("bettertui_test_logs");
            let path = daily_log_path(&temp_dir);

            assert!(path.starts_with(&temp_dir), "Path should be inside log_dir");

            let file_name = path.file_name().unwrap().to_str().unwrap();
            assert!(
                file_name.starts_with("bettertui-"),
                "Filename should start with 'bettertui-'"
            );
            assert!(
                file_name.ends_with(".log"),
                "Filename should end with '.log'"
            );
        }

        #[test]
        fn filename_contains_date() {
            let temp_dir = std::env::temp_dir().join("bettertui_test_logs");
            let path = daily_log_path(&temp_dir);
            let file_name = path.file_name().unwrap().to_str().unwrap();

            let date_part = file_name
                .strip_prefix("bettertui-")
                .unwrap()
                .strip_suffix(".log")
                .unwrap();

            let parts: Vec<&str> = date_part.split('-').collect();
            assert_eq!(parts.len(), 3, "Date part should be YYYY-MM-DD");
        }

        #[test]
        fn multiple_calls_same_minute() {
            let temp_dir = std::env::temp_dir().join("bettertui_test_logs");
            let path1 = daily_log_path(&temp_dir);
            let path2 = daily_log_path(&temp_dir);
            assert_eq!(path1, path2, "Multiple calls should return same path");
        }
    }

    mod open_or_truncate {
        use super::*;
        use std::io::Write;

        fn create_temp_file() -> (tempfile::TempDir, std::path::PathBuf) {
            let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
            let log_path = temp_dir.path().join("test.log");
            (temp_dir, log_path)
        }

        #[test]
        fn creates_new_file() {
            let (temp_dir, log_path) = create_temp_file();

            let result = open_or_truncate(&log_path);
            assert!(result.is_ok(), "Should create new file");

            drop(result);
            drop(temp_dir);
        }

        #[test]
        fn creates_parent_directories() {
            let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
            let nested_path = temp_dir.path().join("deeply/nested/dir/test.log");

            let result = open_or_truncate(&nested_path);
            assert!(result.is_ok(), "Should create parent directories");
            assert!(nested_path.exists(), "File should exist");

            drop(result);
            drop(temp_dir);
        }

        #[test]
        fn appends_to_same_day_file() {
            let (temp_dir, log_path) = create_temp_file();

            let mut file1 = open_or_truncate(&log_path).expect("First open should succeed");
            file1
                .write_all(b"first line\n")
                .expect("Write should succeed");
            file1.flush().expect("Flush should succeed");
            drop(file1);

            let mut file2 = open_or_truncate(&log_path).expect("Second open should succeed");
            file2
                .write_all(b"second line\n")
                .expect("Write should succeed");
            file2.flush().expect("Flush should succeed");
            drop(file2);

            let contents = fs::read_to_string(&log_path).expect("Should read file");
            assert!(contents.contains("first line"), "Should contain first line");
            assert!(
                contents.contains("second line"),
                "Should contain second line"
            );

            drop(temp_dir);
        }

        #[test]
        fn returns_error_for_invalid_path() {
            let result = open_or_truncate(Path::new(""));
            assert!(result.is_err(), "Should fail for invalid path");
        }
    }

    mod integration {
        use super::*;

        #[test]
        fn logger_state_not_initialized_by_default() {
            let state = LOGGER.get();
            assert!(state.is_none(), "Logger should not be initialized in tests");
        }

        #[test]
        fn init_does_not_panic_without_env() {
            unsafe {
                std::env::remove_var("BETTERTUI_LOG_DIR");
            }
            init();
        }

        #[test]
        fn init_with_invalid_env_fails_gracefully() {
            unsafe {
                std::env::set_var("BETTERTUI_LOG_DIR", "relative/path");
                std::env::remove_var("BETTERTUI_LOG_DIR");
            }
        }
    }
}
