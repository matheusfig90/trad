use structopt::StructOpt;

use trad::handlers::echo_handler::EchoHandler;

#[derive(Debug, StructOpt)]
enum Command {
    Echo {
        #[structopt(short = "m", long = "message")]
        message: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Trad-terminal",
    about = "A faster way to interact with TRAD App"
)]
struct CliOpt {
    #[structopt(subcommand)]
    command: Command,

    // -d or --debug, true if it is present
    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

fn main() {
    let app = CliOpt::from_args();
    if app.debug {
        println!("{:?}", app);
    }

    match app.command {
        Command::Echo { message } => {
            let handler = EchoHandler::new(message);
            handler.run();
        }
    }
}
