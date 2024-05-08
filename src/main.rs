use std::io;

mod ter;
mod app;
mod ui;

fn main() -> Result<(), io::Error> {
    let mut terminal = ter::new()?;
    if let Ok(mut app) = app::App::new() {
        if let Err(error) = app.run(&mut terminal) {
            ter::restore()?;
            return Err(error);
        }
    } else {
        ter::restore()?;
        panic!("Can't start app !");
    }
    ter::restore()?;
    Ok(())
}
