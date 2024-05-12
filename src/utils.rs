

/// Checks if a string is in camelCase or PascalCase format.
///
/// # Arguments
///
/// * `name` - A string slice that holds the variable name to be checked.
///
/// # Returns
///
/// A boolean indicating whether the input string is in camelCase or PascalCase format.
///
pub(crate) fn is_camel_or_pascal_case(name: &str) -> bool {
    let mut has_uppercase = false;
    let mut has_lowercase = false;

    let mut chars = name.chars().peekable();

    if let Some(first_char) = chars.peek() {
        if first_char.is_lowercase() {
            has_lowercase = true;
        } else if first_char.is_uppercase() {
            has_uppercase = true;
        }
    }

    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            has_uppercase = true;
        } else if c.is_lowercase() {
            has_lowercase = true;
        } else if c.is_whitespace() || c == '_' {
            // Ignore whitespaces and underscores
            continue;
        }
    }

    has_uppercase && has_lowercase
}

/// Checks if a string starts with a digit.
///
/// # Arguments
///
/// * `name` - A string slice that holds the variable name to be checked.
///
/// # Returns
///
/// A boolean indicating whether the input string starts with a digit.
///
pub(crate) fn starts_with_digit(name: &str) -> bool {
    if let Some(first_char) = name.chars().next() {
        if first_char.is_ascii_digit() {
            return false;
        }
    }
    true
}
