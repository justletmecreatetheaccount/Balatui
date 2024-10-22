use std::io;

mod app;
mod rope;
mod ter;
mod ui;

fn main() -> Result<(), io::Error> {
    //creates a new terminal instance
    let mut terminal = ter::new()?;
    //creates a new app instance (see app.rs for more info)
    if let Ok(mut app) = app::App::new() {
        //runs the app
        if let Err(error) = app.run(&mut terminal) {
            //restores terminal if execution failed
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
