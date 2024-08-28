use uuid::Uuid;

use crate::errors::GnapError;

/// this ensures each model type that will be cached provides a
/// consistent path to cache objects
pub trait CachePath {
    fn cache_path() -> &'static str;
}

/// this ensures any model that contains an "id", such as "client_id"
/// is generated and parsed in a consistent manner
pub trait GnapID {
    fn parse_id(&self) -> Result<Uuid, GnapError>;
}