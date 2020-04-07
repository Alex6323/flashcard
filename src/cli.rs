use structopt::StructOpt;

/// A CLI for this application.
pub struct Cli
{
    args: Args,
}

#[derive(StructOpt)]
struct Args
{
    #[structopt(short, long, help = "file path to the cardbox")]
    filepath: String,
}

impl Cli
{
    /// Create a new command line interface.
    pub fn new() -> Self
    {
        Self { args: Args::from_args() }
    }

    /// Returns the filepath to the cardbox.
    pub fn filepath(&self) -> &str
    {
        &self.args.filepath
    }
}
