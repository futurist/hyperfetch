pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

pub fn format_speed(bytes_per_sec: f64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec as u64))
}

pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

pub fn extract_filename_from_url(url: &str) -> String {
    url::Url::parse(url)
        .ok()
        .and_then(|u| {
            u.path_segments()
                .and_then(|segments| segments.last())
                .filter(|name| !name.is_empty())
                .map(|name| name.to_string())
        })
        .unwrap_or_else(|| "downloaded_file".to_string())
}

pub fn parse_size(size_str: &str) -> Result<u64, anyhow::Error> {
    let size_str = size_str.trim().to_uppercase();

    let (number_part, suffix) = if size_str.ends_with('K') {
        (&size_str[..size_str.len() - 1], 1024u64)
    } else if size_str.ends_with('M') {
        (&size_str[..size_str.len() - 1], 1024u64.pow(2))
    } else if size_str.ends_with('G') {
        (&size_str[..size_str.len() - 1], 1024u64.pow(3))
    } else if size_str.ends_with('T') {
        (&size_str[..size_str.len() - 1], 1024u64.pow(4))
    } else {
        (size_str.as_str(), 1u64)
    };

    let number: f64 = number_part.parse()?;
    Ok((number * suffix as f64) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_bytes() {
        assert_eq!(parse_size("1024").unwrap(), 1024);
        assert_eq!(parse_size("100").unwrap(), 100);
    }

    #[test]
    fn test_parse_size_kilobytes() {
        assert_eq!(parse_size("1K").unwrap(), 1024);
        assert_eq!(parse_size("2K").unwrap(), 2048);
        assert_eq!(parse_size("1k").unwrap(), 1024); // lowercase
    }

    #[test]
    fn test_parse_size_megabytes() {
        assert_eq!(parse_size("1M").unwrap(), 1024 * 1024);
        assert_eq!(parse_size("2M").unwrap(), 2 * 1024 * 1024);
        assert_eq!(parse_size("1m").unwrap(), 1024 * 1024); // lowercase
    }

    #[test]
    fn test_parse_size_gigabytes() {
        assert_eq!(parse_size("1G").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(parse_size("2G").unwrap(), 2 * 1024 * 1024 * 1024);
        assert_eq!(parse_size("1g").unwrap(), 1024 * 1024 * 1024); // lowercase
    }

    #[test]
    fn test_parse_size_terabytes() {
        assert_eq!(parse_size("1T").unwrap(), 1024u64 * 1024 * 1024 * 1024);
        assert_eq!(parse_size("1t").unwrap(), 1024u64 * 1024 * 1024 * 1024); // lowercase
    }

    #[test]
    fn test_parse_size_with_whitespace() {
        assert_eq!(parse_size("  1M  ").unwrap(), 1024 * 1024);
        assert_eq!(parse_size(" 100 ").unwrap(), 100);
    }

    #[test]
    fn test_parse_size_invalid() {
        assert!(parse_size("invalid").is_err());
        assert!(parse_size("1X").is_err());
        assert!(parse_size("").is_err());
    }

    #[test]
    fn test_format_bytes_basic() {
        assert_eq!(format_bytes(0), "0.00 B");
        assert_eq!(format_bytes(512), "512.00 B");
        assert_eq!(format_bytes(1023), "1023.00 B");
    }

    #[test]
    fn test_format_bytes_kilobytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(2048), "2.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
    }

    #[test]
    fn test_format_bytes_megabytes() {
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(2 * 1024 * 1024), "2.00 MB");
        assert_eq!(format_bytes(1536 * 1024), "1.50 MB");
    }

    #[test]
    fn test_format_bytes_gigabytes() {
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
        assert_eq!(format_bytes(2 * 1024 * 1024 * 1024), "2.00 GB");
    }

    #[test]
    fn test_format_bytes_terabytes() {
        assert_eq!(format_bytes(1024u64 * 1024 * 1024 * 1024), "1.00 TB");
        assert_eq!(format_bytes(2 * 1024u64 * 1024 * 1024 * 1024), "2.00 TB");
    }

    #[test]
    fn test_format_duration_seconds_only() {
        assert_eq!(format_duration(0), "0s");
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(59), "59s");
    }

    #[test]
    fn test_format_duration_minutes_and_seconds() {
        assert_eq!(format_duration(60), "1m 0s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3599), "59m 59s");
    }

    #[test]
    fn test_format_duration_hours_minutes_seconds() {
        assert_eq!(format_duration(3600), "1h 0m 0s");
        assert_eq!(format_duration(3661), "1h 1m 1s");
        assert_eq!(format_duration(7325), "2h 2m 5s");
    }

    #[test]
    fn test_extract_filename_from_url_with_filename() {
        assert_eq!(
            extract_filename_from_url("http://example.com/file.zip"),
            "file.zip"
        );
        assert_eq!(
            extract_filename_from_url("https://example.com/path/to/document.pdf"),
            "document.pdf"
        );
    }

    #[test]
    fn test_extract_filename_from_url_without_path() {
        assert_eq!(
            extract_filename_from_url("http://example.com"),
            "downloaded_file"
        );
        assert_eq!(
            extract_filename_from_url("http://example.com/"),
            "downloaded_file"
        );
    }

    #[test]
    fn test_extract_filename_from_url_with_query() {
        assert_eq!(
            extract_filename_from_url("http://example.com/file.zip?download=true"),
            "file.zip"
        );
    }

    #[test]
    fn test_extract_filename_from_url_invalid() {
        assert_eq!(
            extract_filename_from_url("not a url"),
            "downloaded_file"
        );
    }

    #[test]
    fn test_format_speed() {
        assert_eq!(format_speed(0.0), "0.00 B/s");
        assert_eq!(format_speed(1024.0), "1.00 KB/s");
        assert_eq!(format_speed(1048576.0), "1.00 MB/s");
    }
}
