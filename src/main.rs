use clap::Parser;
use uuid::Uuid;

fn main() {
    let args = Args::parse();

    if let Some(guid) = args.guid_to_long {
        let long = guid.as_u128();
        println!("Guid: {}\nLong: {}", guid, long);
    } else if let Some(long) = args.long_to_guid {
        let guid = Uuid::from_u128(long);
        println!("Long: {}\nGuid: {}", long, guid);
    } else if let Some(value) = args.value {
        // Try parsing as UUID first
        if let Ok(guid) = value.parse::<Uuid>() {
            let long = guid.as_u128();
            println!("Guid: {}\nLong: {}", guid, long);
        }
        // Try parsing as u128
        else if let Ok(long) = value.parse::<u128>() {
            let guid = Uuid::from_u128(long);
            println!("Long: {}\nGuid: {}", long, guid);
        }
        // Neither worked
        else {
            eprintln!(
                "Error: Unable to parse '{}' as either a UUID or a number",
                value
            );
            std::process::exit(1);
        }
    } else {
        panic!("Missing arguments!");
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub guid_to_long: Option<Uuid>,

    #[arg(short, long)]
    pub long_to_guid: Option<u128>,

    /// The value to convert (either a UUID or a number)
    pub value: Option<String>,
}
