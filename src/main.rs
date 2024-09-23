mod cli;
use chrono::{DateTime, FixedOffset, Utc};
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

fn print_outage_events(events: &[OutageEvent], title: &str, is_today: bool) {
    if events.is_empty() && is_today {
        println!("There are no planned outages scheduled for today. 🎉");
        return;
    }

    if events.is_empty() && !is_today {
        println!("There are no planned outages scheduled for tomorrow. 🎉");
        return;
    }

    let mut table = Table::new();
    let utc_plus_4 = FixedOffset::east_opt(4 * 3600).expect("Unable to create UTC+4 offset");
    let now = Utc::now().with_timezone(&utc_plus_4);

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let mut headers = vec![];
    if is_today {
        headers.push(Cell::new("Status").fg(Color::Green));
    } else {
        headers.push(Cell::new("Date").fg(Color::Green));
    }
    headers.extend(vec![
        Cell::new("Location").fg(Color::Green),
        Cell::new("Streets").fg(Color::Green),
        Cell::new("Time").fg(Color::Green),
    ]);
    table.set_header(headers);

    for event in events {
        let mut row = vec![];

        if is_today {
            let is_ongoing = event.from_time <= now && now <= event.to_time;
            let status = if is_ongoing { "🔴" } else { "🟢" };
            row.push(Cell::new(status));
        } else {
            row.push(Cell::new(&event.date).fg(Color::Yellow));
        }

        row.extend(vec![
            Cell::new(format!("{} ({})", &event.locality, &event.district)),
            Cell::new(&event.streets),
            Cell::new(format!(
                "{} - {}",
                event.from_time.with_timezone(&utc_plus_4).format("%H:%M"),
                event.to_time.with_timezone(&utc_plus_4).format("%H:%M")
            ))
            .fg(Color::Cyan),
        ]);

        table.add_row(row);
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

    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("today", _)) => {
            print_outage_events(&outage.today, "Today's Outages", true);
        }
        Some(("tomorrow", _)) => {
            print_outage_events(&outage.future, "Future Outages", false);
        }
        Some(("all", _)) => {
            print_outage_events(&outage.today, "Today's Outages", true);
            if !outage.today.is_empty() {
                println!("\n");
            }
            print_outage_events(&outage.future, "Future Outages", false);
        }
        _ => {
            println!("Please use 'today', 'tomorrow', or 'all' subcommand.");
            std::process::exit(1);
        }
    }

    Ok(())
}
