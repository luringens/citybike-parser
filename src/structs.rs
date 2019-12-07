use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub capacity: u8,
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
    pub is_installed: u8,
    pub is_renting: u8,
    pub is_returning: u8,
    pub last_reported: i64,
    pub num_bikes_available: u8,
    pub num_docks_available: u8,
}

#[derive(Serialize, Debug)]
pub struct Entry {
    pub timestamp: String,
    pub station_id: u32,
    pub num_bikes_available: u8,
}

#[derive(Serialize, Debug)]
pub struct Station {
    pub station_name: String,
    pub station_id: u32,
    pub lat: f64,
    pub lon: f64,
    pub capacity: u8,
    pub bikes_available: HashMap<String, u8>,
}
