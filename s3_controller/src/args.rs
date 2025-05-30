use clap::{arg, command, Parser};

fn bucket() -> String {
    "dz-bucket-1234".to_string()
}

fn region() -> String {
    "us-east-1".to_string().to_string()
}

fn tls() -> bool {
    false
}

fn key_path_tls() -> String {
    "".to_string()
}
fn cert_path_tls() -> String {
    "".to_string()
}
fn port() -> i16 {
    5000
}

#[derive(Parser, Debug)]
#[command(name = "s3viewer")]
pub struct Args {
    #[arg(short, long, default_value_t = bucket())]
    pub bucket: String,

    #[arg(short, long, default_value_t = region())]
    pub region: String,

    #[arg(short, long, default_value_t = key_path_tls())]
    pub key_path_tls: String,

    #[arg(short, long, default_value_t = cert_path_tls())]
    pub cert_path_tls: String,

    #[arg(short, long, default_value_t = tls())]
    pub tls: bool,

    #[arg(short, long, default_value_t = port())]
    pub port: i16,
}
