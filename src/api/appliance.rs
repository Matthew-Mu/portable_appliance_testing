use crate::model::appliance::{Environment, Private};
use crate::repository::ddb::DDBRepository;
use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use chrono::NaiveDate;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct SubmitPrivateRequest {
    name: String,
    description: String,
    ir: String,
    resistance_to_earth: String,
    voltage: String,
    tested_by: String,
    tag_number: String,
    environment: Environment,
    date: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApplianceIdentifier {
    tag_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct DateIdentifier {
    date: String,
}

#[derive(Debug, Display)]
pub enum ApplianceError {
    ApplianceNotFound,
    ApplianceUpdateFailure,
    ApplianceCreationFailure,
    BadApplianceRequest,
}

impl ResponseError for ApplianceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApplianceError::ApplianceNotFound => StatusCode::NOT_FOUND,
            ApplianceError::ApplianceUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            ApplianceError::ApplianceCreationFailure => StatusCode::FAILED_DEPENDENCY,
            ApplianceError::BadApplianceRequest => StatusCode::BAD_REQUEST,
        }
    }
}

#[post("/private")]
pub async fn submit_private(
    ddb_repo: Data<DDBRepository>,
    request: Json<SubmitPrivateRequest>,
) -> Result<Json<ApplianceIdentifier>, ApplianceError> {
    let private = Private::new(
        request.name.clone(),
        request.description.clone(),
        request.ir.clone().parse::<f32>().unwrap(),
        request.resistance_to_earth.clone().parse::<f32>().unwrap(),
        request.voltage.clone().parse::<i64>().unwrap(),
        request.tested_by.clone(),
        request.tag_number.clone(),
        request.environment.clone(),
        Private::from_str(request.date.clone()),
    );

    let private_identifier = private.get_tag_number();
    match ddb_repo.put_private(private).await {
        Ok(()) => Ok(Json(ApplianceIdentifier {
            tag_number: private_identifier,
        })),
        Err(_) => Err(ApplianceError::ApplianceCreationFailure),
    }
}

#[get("/PrivApp")]
pub async fn get_previous_appliances(
    ddb_repo: Data<DDBRepository>,
) -> Result<Json<Vec<Private>>, ApplianceError> {
    let vec_of_private = ddb_repo.read_private_appliances().await;
    match vec_of_private {
        Some(vec_of_private) => Ok(Json(vec_of_private)),
        None => Err(ApplianceError::ApplianceNotFound),
    }
}

#[get("/240")]
pub async fn get_240(ddb_repo: Data<DDBRepository>) -> Result<Json<Vec<Private>>, ApplianceError> {
    let vec_of_240 = ddb_repo.get_240_v().await;
    match vec_of_240 {
        Some(vec_of_private) => Ok(Json(vec_of_private)),
        None => Err(ApplianceError::ApplianceNotFound),
    }
}

#[get("/115")]
pub async fn get_115(ddb_repo: Data<DDBRepository>) -> Result<Json<Vec<Private>>, ApplianceError> {
    let vec_of_115 = ddb_repo.get_115_v().await;
    match vec_of_115 {
        Some(vec_of_private) => Ok(Json(vec_of_private)),
        None => Err(ApplianceError::ApplianceNotFound),
    }
}

#[get("/outOfDate/{date}")]
pub async fn get_out_of_date(
    string_date: Path<DateIdentifier>,
    ddb_repo: Data<DDBRepository>,
) -> Result<Json<Vec<Private>>, ApplianceError> {
    let date = NaiveDate::from_str(string_date.into_inner().date.as_str()).unwrap();
    let vec_out_of_date = ddb_repo.out_of_date(date).await;
    match vec_out_of_date {
        Some(vec_out_of_date) => Ok(Json(vec_out_of_date)),
        None => Err(ApplianceError::ApplianceNotFound),
    }
}
