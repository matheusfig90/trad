mod handlers;
mod main_app;

use handlers::echo_handler::EchoHandler;
use main_app::App;
use std::io;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
enum Command {
    Echo {
        #[structopt(short = "m", long = "message")]
        message: String,
    },
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "Trad")]
struct MainArgs {
    #[structopt(subcommand)]
    command: Option<Command>,

    // -d or --debug, true if it is present
    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

fn main() -> Result<(), io::Error> {
    let args = MainArgs::from_args();
    if args.debug {
        println!("{:?}", args);
    }

    if args.command.is_none() {
        let mut app = App::new();

        return app.run();
    }

    match args.command.unwrap() {
        Command::Echo { message } => {
            let handler = EchoHandler::new(message);
            handler.run();

            Ok(())
        }
    }
}
