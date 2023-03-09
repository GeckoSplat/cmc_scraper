use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub start: String,

    #[arg(long)]
    pub limit: String,

    #[arg(long)]
    pub convert: String,
}
