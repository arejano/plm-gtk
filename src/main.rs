use anyhow::Result;

use relm4::RelmApp;

use app::App;

use application::setup;
mod app;
mod application;

fn main() -> Result<()> {
    let main_app = setup::init()?;
    let app = RelmApp::from_app(main_app);
    app.run_async::<App>(());
    Ok(())
}
