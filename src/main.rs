use clap::Parser;

#[derive(Parser)]
#[command(name="aos-s")]
#[command(author="Alexandre Figueiredo")]
#[command(version="0.0.1")]
#[command(about="Interacts with Aruba switches API", long_about=None)]
struct Cli {
    #[arg(short, long)]
    ip: String,

    #[arg(short, long)]
    port: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    get_macs(cli.ip, cli.port)
}

fn get_macs(ip: String, port: u8) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}/rest/v8/ports/{}/mac-table", ip, port);
    let resp = reqwest::blocking::get(url)?
        .json::<serde_json::Value>();
    println!("{:#?}", resp);
    Ok(())
}

