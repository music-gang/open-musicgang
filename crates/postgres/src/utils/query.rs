/// Return a formatted limit offset clause.
/// For example:
/// format_limit_offset(10, 0) => "LIMIT 10"
/// ```
/// use openmusicgang_postgres::utils::query::*;
/// let limit = 10;
/// let offset = 0;
/// let clause = format_limit_offset(limit, offset);
/// assert_eq!(clause, "LIMIT 10")
/// ```
pub fn format_limit_offset(limit: i64, offset: i64) -> String {
    if limit > 00 && offset > 00 {
        return format!("LIMIT {} OFFSET {}", limit, offset);
    } else if limit > 00 {
        return format!("LIMIT {}", limit);
    } else if offset > 00 {
        return format!("OFFSET {}", offset);
    } else {
        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_limit_offset() {
        let query = format_limit_offset(40, 0);
        assert_eq!(query, "LIMIT 40");

        let query = format_limit_offset(0, 40);
        assert_eq!(query, "OFFSET 40");

        let query = format_limit_offset(40, 40);
        assert_eq!(query, "LIMIT 40 OFFSET 40");

        let query = format_limit_offset(0, 0);
        assert_eq!(query, "");
    }
}
