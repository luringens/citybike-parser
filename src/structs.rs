use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct JsStationFile {
    pub last_updated: u32,
    pub ttl: u32,
    pub data: JsStations,
}

#[derive(Deserialize, Debug)]
pub struct JsStations {
    pub stations: Vec<JsStation>,
}

#[derive(Deserialize, Debug)]
pub struct JsStation {
    pub station_id: String,
    pub name: String,
    pub address: String,
    pub lat: f64,
    pub lon: f64,
    pub capacity: u32,
}

#[derive(Deserialize, Debug)]
pub struct JsStatusFile {
    pub last_updated: u32,
    pub ttl: u32,
    pub data: JsStatuses,
}

#[derive(Deserialize, Debug)]
pub struct JsStatuses {
    pub stations: Vec<JsStatus>,
}

#[derive(Deserialize, Debug)]
pub struct JsStatus {
    pub station_id: String,
    pub is_installed: u32,
    pub is_renting: u32,
    pub is_returning: u32,
    pub last_reported: i64,
    pub num_bikes_available: u32,
    pub num_docks_available: u32,
}

#[derive(Debug)]
pub struct Entry {
    pub timestamp: NaiveDateTime,
    pub station_id: u32,
    pub num_bikes_available: u32,
}

#[derive(Serialize, Debug)]
pub struct Station {
    pub station_name: String,
    pub station_id: u32,
    pub lat: f64,
    pub lon: f64,
    pub capacity: u32,
    pub bikes_available: Vec<StationStatusentry>,
}

#[derive(Serialize, Debug)]
pub struct StationStatusentry {
    pub time: u32,
    pub bikes: f32,
}
