mod structs;

use chrono::NaiveDateTime;
use pbr::ProgressBar;
use std::collections::HashMap;

use structs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 4);

    // Parse station file.
    let station_filename = std::fs::read_to_string(&args[2])?;
    let stations: std::collections::HashMap<u32, JsStation> =
        serde_json::from_str::<JsStationFile>(&station_filename)?
            .data
            .stations
            .into_iter()
            .map(|station| {
                (
                    str::parse(&station.station_id).expect("Station ID is not u32"),
                    station,
                )
            })
            .collect();

    // Parse status files
    let mut out_stations: HashMap<u32, Station> =
        HashMap::with_capacity((args.len() - 1) * stations.len());
    let dir: Vec<_> = std::fs::read_dir(&args[3])?.collect();
    let mut pb = ProgressBar::new(dir.len() as u64);
    pb.format("[=>-]");

    for file in dir {
        let status_filename = std::fs::read_to_string(file?.path())?;
        let entries = serde_json::from_str::<JsStatusFile>(&status_filename)?
            .data
            .stations
            .into_iter()
            .map(|status| Entry {
                timestamp: NaiveDateTime::from_timestamp(status.last_reported, 0)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                station_id: str::parse(&status.station_id).expect("Station ID is not u32"),
                num_bikes_available: status.num_bikes_available,
            });
        for e in entries {
            let s = &stations[&e.station_id];
            out_stations
                .entry(e.station_id)
                .or_insert(Station {
                    station_id: e.station_id,
                    station_name: s.name.clone(),
                    lat: s.lat,
                    lon: s.lon,
                    capacity: s.capacity,
                    bikes_available: HashMap::new(),
                })
                .bikes_available
                .insert(e.timestamp.clone(), e.num_bikes_available);
        }
        pb.inc();
    }

    let data = serde_json::to_string(&out_stations.values().collect::<Vec<_>>())?;
    std::fs::write(&args[1], data)?;
    pb.finish_print("done");

    Ok(())
}
