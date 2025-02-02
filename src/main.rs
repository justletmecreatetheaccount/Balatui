use color_eyre::Result;

mod app;
mod modes;
mod popup;
mod rope;
mod term;

fn main() -> Result<()> {
    color_eyre::install()?;
    //creates a new crossterm terminal instance
    if let Ok(mut terminal) = term::init() {
        //creates a new app instance (see app.rs for more info)
        if let Ok(mut app) = app::App::new() {
            //runs the app
            if let Err(error) = app.run(&mut terminal) {
                //restores terminal if execution failed
                term::restore();
                return Err(error);
            }
        }
    } else {
        term::restore();
        panic!("Can't start app !");
    }
    term::restore();
    Ok(())
}
