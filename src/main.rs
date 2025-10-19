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
}
