use clap::{arg, command, Parser};

fn bucket() -> String {
    "dz-bucket-1234".to_string()
}

fn region() -> String {
    "us-east-1".to_string().to_string()
}

#[derive(Parser, Debug)]
#[command(name = "s3viewer")]
pub struct Args {
    #[arg(short, long, default_value_t = bucket())]
    pub bucket: String,

    #[arg(short, long, default_value_t = region())]
    pub region: String,
}
