mod main_app;

use main_app::App;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut app = App::new();

    app.run()
}
