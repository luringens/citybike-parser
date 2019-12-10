//mod errors;
mod structs;

use chrono::prelude::*;
use pbr::ProgressBar;
use std::collections::HashMap;

use structs::*;

fn main() {
    if let Err(e) = do_main() {
        use color_backtrace::{failure::print_backtrace, Settings};
        unsafe {
            print_backtrace(&e.backtrace(), &mut Settings::new()).unwrap();
        }
    }
}

fn do_main() -> failure::Fallible<()> {
    color_backtrace::install();
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
    let mut out_stations: [HashMap<u32, Station>; 7] = Default::default();
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
                timestamp: NaiveDateTime::from_timestamp(status.last_reported, 0),
                station_id: str::parse(&status.station_id).expect("Station ID is not u32"),
                num_bikes_available: status.num_bikes_available,
            });
        for e in entries {
            let s = &stations[&e.station_id];
            let day_of_week = e.timestamp.weekday().num_days_from_monday() as usize;
            out_stations[day_of_week]
                .entry(e.station_id)
                .or_insert(Station {
                    station_id: e.station_id,
                    station_name: s.name.clone(),
                    lat: s.lat,
                    lon: s.lon,
                    capacity: s.capacity,
                    bikes_available: Default::default(),
                })
                .bikes_available
                .push(StationStatusentry {
                    time: e.timestamp.hour(),
                    bikes: e.num_bikes_available as f32,
                });
        }
        pb.inc();
    }

    let mut result: [Vec<Station>; 7] = Default::default();
    for (i, e) in out_stations.iter_mut().enumerate() {
        //result[i] = *e.into_iter().map(|(_, v)| *v).collect::<Vec<Station>>();
        result[i] = e.drain().map(|(_, v)| v).collect::<Vec<Station>>();
    }

    for e in result.iter_mut().flat_map(|i| i.iter_mut()) {
        let mut newbikes: HashMap<u32, (f32, usize)> =
            HashMap::with_capacity(e.bikes_available.len());
        for status in e.bikes_available.iter() {
            let entry = newbikes.entry(status.time).or_insert((0.0, 0));
            entry.0 += status.bikes;
            entry.1 += 1;
        }
        let mut newbikes: Vec<_> = newbikes
            .into_iter()
            .map(|(time, (bikes, n))| (time, bikes / n as f32))
            .map(|(time, bikes)| StationStatusentry { time, bikes })
            .collect();
        newbikes.sort_unstable_by_key(|e| e.time);
        e.bikes_available.clear();

        e.bikes_available = newbikes;
    }

    let data = serde_json::to_string(&result)?;
    std::fs::write(&args[1], data)?;
    pb.finish_print("done");

    Ok(())
}
