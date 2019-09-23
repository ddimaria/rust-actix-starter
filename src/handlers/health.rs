use crate::errors::ApiError;
use crate::helpers::respond_json;
use actix_web::web::Json;

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Handler to get the liveness of the service
pub fn get_health() -> Result<Json<HealthResponse>, ApiError> {
    respond_json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}
