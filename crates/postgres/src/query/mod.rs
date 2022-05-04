pub mod user;

/// Returns a a string as single where condition for an equality comparison.
///
/// # Arguments
/// * `field` - The field to compare.
/// * `param` - The numbered parameter to use in the where condition.
///
/// # Returns
///
/// A string as single where condition for an equality comparison.
///
/// # Example
/// ```
/// use openmusicgang_postgres::where_condition_eq;
/// assert_eq!(where_condition_eq!("id", 1), "id = $1");
/// ```
#[macro_export]
macro_rules! where_condition_eq {
    ($field:expr, $param:expr) => {
        format!("{} = ${}", $field, $param)
    };
}

/// Returns a string as single where condition for a not equal comparison.
///
/// # Arguments
/// * `field` - The field to compare.
/// * `param` - The numbered parameter to use in the query.
///
/// # Returns
///
/// A string as single where condition for a not equal comparison.
///
/// # Example usage
/// ```
/// use openmusicgang_postgres::where_condition_ne;
/// assert_eq!(where_condition_ne!("id", 1), "id != $1");
/// ```
#[macro_export]
macro_rules! where_condition_ne {
    ($field:expr, $param:expr) => {
        format!("{} != ${}", $field, $param)
    };
}

/// Returns a formatted limit offset clause.
///
/// # Arguments
/// * `limit` - The limit to use in the query.
/// * `offset` - The offset to use in the query.
///
/// # Returns
/// A string with the limit and offset clause.
///
/// # Example usage
/// ```
/// use openmusicgang_postgres::format_limit_offset;
/// assert_eq!(format_limit_offset!(10, 0), "LIMIT 10");
/// assert_eq!(format_limit_offset!(10, 1), "LIMIT 10 OFFSET 1");
/// assert_eq!(format_limit_offset!(0, 1), "OFFSET 1");
/// assert_eq!(format_limit_offset!(0, 0), "");
/// ```
#[macro_export]
macro_rules! format_limit_offset {
    ($limit:expr, $offset:expr) => {
        if $limit > 00 && $offset > 00 {
            format!("LIMIT {} OFFSET {}", $limit, $offset)
        } else if $limit > 00 {
            format!("LIMIT {}", $limit)
        } else if $offset > 00 {
            format!("OFFSET {}", $offset)
        } else {
            "".to_string()
        }
    };
}
