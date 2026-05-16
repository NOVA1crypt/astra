use url::Url;

pub const HOME: &str = "astra://home";
const SEARCH_BASE: &str = "https://search.brave.com/search?q=";

pub fn resolve(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return HOME.to_string();
    }
    if let Ok(parsed) = Url::parse(trimmed) {
        if matches!(parsed.scheme(), "http" | "https" | "file" | "astra") {
            return trimmed.to_string();
        }
    }
    if !trimmed.contains(' ') && trimmed.contains('.') && !trimmed.starts_with('-') {
        return format!("https://{}", trimmed);
    }
    let encoded = url::form_urlencoded::byte_serialize(trimmed.as_bytes()).collect::<String>();
    format!("{}{}", SEARCH_BASE, encoded)
}
