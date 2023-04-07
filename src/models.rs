use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Type};
use utoipa::ToSchema;

#[derive(Clone)]
pub struct AppState {
    pub database: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Machine {
    pub machine_id: String,
    pub room_id: i32,
    pub machine_type: MachineType,
}

#[derive(Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "machine_type", rename_all = "lowercase")]
pub enum MachineType {
    Washer,
    Dryer,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Room {
    pub room_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    pub username: String,
    pub admin: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Report {
    pub report_id: i32,
    pub room_id: i32,
    pub machine_id: String,
    pub reporter_username: String,
    pub report_type: ReportType,
    pub time: NaiveDateTime,
    pub description: Option<String>,
    pub archived: bool,
}

#[derive(Debug, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "report_type", rename_all = "lowercase")]
pub enum ReportType {
    Operational,
    Caution,
    Broken,
}
