pub fn format_bytes(bytes: i64) -> String {
    if bytes == 0 {
        return format!("0 Bytes");
    }
    let k: f64 = 1024.0;
    const SIZES: [&str; 9] = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let bytes = bytes as f64;
    let i = (bytes.ln() / k.ln())
        .floor() as i32;

    format!("{:.2} {}", (bytes / k.powi(i)), SIZES[i as usize])
}