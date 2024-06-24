pub fn validate_field(field: &Option<String>, error_message: &str) -> Result<(), String> {
    if let Some(field) = field {
        if field.trim().is_empty() {
            return Err(error_message.to_string());
        }
    }

    Ok(())
}
