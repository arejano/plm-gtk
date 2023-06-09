use anyhow::{Ok, Result};
use once_cell::unsync::Lazy;
use relm4::{adw, gtk::gio};

use crate::application::info::APP_ID;

thread_local! {
    static APP: Lazy<adw::Application> = Lazy::new(|| { adw::Application::new(Some(APP_ID), gio::ApplicationFlags::empty())});
}

pub fn main_app() -> adw::Application {
    APP.with(|app| (*app).clone())
}

pub fn init() -> Result<adw::Application> {
    let app = main_app();

    gtk::init()?;
    Ok(app)
}
