use clap::Command;

pub fn cli() -> Command {
    Command::new("courant")
        .version("0.1.0")
        .author("Jain Ramchurn")
        .subcommand(
            Command::new("today")
                .about("Show today's power outages")
                .aliases(["zordi"]),
        )
        .subcommand(
            Command::new("tomorrow")
                .about("Show tomorrow's power outages")
                .aliases(["demain", "dmain", "future"]),
        )
        .subcommand(
            Command::new("all")
                .about("Show both today's and tomorrow's power outages")
                .aliases(["tou", "tout"]),
        )
}
