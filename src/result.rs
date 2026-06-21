pub type Result<T> = std::result::Result<T, crate::AppError>;
pub type OrmResult<T> = std::result::Result<T, crate::error::orm::OrmError>;
