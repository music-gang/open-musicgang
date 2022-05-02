use core::fmt;

/// ErrorCode is an enum to represent error codes.
/// You can define your own error codes here.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ErrorCode {
    EINTERNAL,
    EINVALID,
    EUNAUTHORIZED,
    EFORBIDDEN,
    EUNKNOWN,
    ENOTFOUND,
    ECONFLICT,
    ENOTIMPLEMENTED,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ErrorCode {
    /// Returns the error code as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::EINTERNAL => "internal server error",
            ErrorCode::EINVALID => "invalid",
            ErrorCode::EFORBIDDEN => "forbidden",
            ErrorCode::EUNKNOWN => "unknown",
            ErrorCode::ENOTFOUND => "not found",
            ErrorCode::ECONFLICT => "conflict",
            ErrorCode::EUNAUTHORIZED => "unauthorized",
            ErrorCode::ENOTIMPLEMENTED => "not implemented",
        }
    }

    /// Returns the error code as an http status code.
    pub fn as_http_status(&self) -> u16 {
        match self {
            ErrorCode::EINTERNAL => 500,
            ErrorCode::EINVALID => 400,
            ErrorCode::EFORBIDDEN => 403,
            ErrorCode::EUNKNOWN => 500,
            ErrorCode::ENOTFOUND => 404,
            ErrorCode::ECONFLICT => 409,
            ErrorCode::EUNAUTHORIZED => 401,
            ErrorCode::ENOTIMPLEMENTED => 501,
        }
    }
}

/// Error is a struct to represent an error that occurred in the application.
/// Error is considered a managed error and all errors exchanged in the application should be of this type.
#[derive(Debug, PartialEq)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code='{}' message='{}'", self.code, self.message)
    }
}

impl Error {
    pub fn new(code: ErrorCode, message: String) -> Error {
        Error { code, message }
    }
}

mod tests {

    #[test]
    fn new_error() {
        let error = super::Error::new(
            super::ErrorCode::EINTERNAL,
            "internal server error".to_string(),
        );

        assert_eq!(error.code, super::ErrorCode::EINTERNAL);
        assert_eq!(error.message, "internal server error");

        let error = super::Error::new(super::ErrorCode::EINVALID, "invalid".to_string());

        assert_eq!(error.code, super::ErrorCode::EINVALID);
        assert_eq!(error.message, "invalid");
    }
}
