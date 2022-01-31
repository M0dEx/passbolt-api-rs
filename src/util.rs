/// Formats a template string using simple replacen
pub fn format(template: &str, values: &[&str]) -> String {
    let mut result = template.to_string();

    for value in values {
        result = result.replacen("{}", *value, 1);
    }

    result
}
