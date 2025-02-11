use clap::Args;
use std::error::Error;

#[derive(Args)]
pub struct Arguments {
    #[arg(default_value = "Friend")]
    name: String,
}

pub fn run(arguments: &Arguments) -> Result<(), Box<dyn Error>> {
    tracing::info!("Hello {}, i am parquetify", arguments.name);
    Ok(())
}
