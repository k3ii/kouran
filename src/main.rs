use chrono::{DateTime, Utc};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::ContentArrangement;
use comfy_table::{Cell, Color, Table};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct PowerOutageData {
    today: Vec<OutageEvent>,
    future: Vec<OutageEvent>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutageEvent {
    date: String,
    locality: String,
    streets: String,
    district: String,
    #[serde(rename = "from")]
    from_time: DateTime<Utc>,
    #[serde(rename = "to")]
    to_time: DateTime<Utc>,
    id: String,
}

fn print_outage_events(events: &[OutageEvent], title: &str) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        //        .set_width(100)
        .set_header(vec![
            Cell::new("Date").fg(Color::Green),
            Cell::new("Locality").fg(Color::Green),
            Cell::new("Streets").fg(Color::Green),
            Cell::new("District").fg(Color::Green),
            Cell::new("From").fg(Color::Green),
            Cell::new("To").fg(Color::Green),
        ]);

    for event in events {
        table.add_row(vec![
            Cell::new(&event.date).fg(Color::Yellow),
            Cell::new(&event.locality),
            Cell::new(&event.streets),
            Cell::new(&event.district),
            Cell::new(event.from_time.format("%H:%M").to_string()).fg(Color::Cyan),
            Cell::new(event.to_time.format("%H:%M").to_string()).fg(Color::Cyan),
        ]);
    }

    println!("{}", title);
    println!("{}", table);
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let outage: PowerOutageData = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/MrSunshyne/mauritius-dataset-electricity/main/data/power-outages.latest.json")
        .send()
        .await?
        .json()
        .await?;

    print_outage_events(&outage.today, "Today's Outages");
    print_outage_events(&outage.future, "Future Outages");

    Ok(())
}
