use clap::{command, Parser};
use gtk4::{
    glib::ExitCode,
    prelude::{ApplicationExt, ApplicationExtManual},
};
use std::{path::PathBuf, rc::Rc};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,
}

const APP_ID: &str = "dev.takimoysha.simple-linux-widget";

pub mod config {
    fn default_title() -> String {
        String::from("Simple Linux Widget")
    }

    #[derive(serde::Deserialize)]
    pub struct Config {
        #[serde(default = "default_title")]
        pub title: String,
        pub style: Style,
    }

    #[derive(serde::Deserialize)]
    pub struct Style {
        color_bg: Option<String>,
    }

    pub fn load_config(path: std::path::PathBuf) -> Config {
        let raw_config = &std::fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(raw_config).expect("Failed to parse config file")
    }
}

pub mod ui {
    use gtk4::prelude::*;

    pub mod css {
        use glib::current_dir;
        use gtk4::{gdk::Display, CssProvider};

        pub fn load_css(config: &crate::config::Config) {
            // Load the CSS file and add it to the provider
            let provider = gtk4::CssProvider::new();
            provider.load_from_path(current_dir().join("assets/app.css"));

            gtk4::style_context_add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    pub fn build_ui(app: &gtk4::Application, config: &crate::config::Config) {
        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .title(&config.title)
            .default_width(200)
            .default_height(200)
            .build();

        window.set_decorated(false);
        window.set_resizable(false);
        window.set_default_size(375, 200);

        let vbox = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .build();

        vbox.set_css_classes(&["container"]);
        window.set_child(Some(&vbox));

        vbox.set_tooltip_text(Some(&config.title));

        window.present();
    }
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let config = Rc::new(match args.config {
        Some(config_path) => config::load_config(PathBuf::from(config_path)),
        None => config::load_config(PathBuf::from("configs/default.toml")),
    });
    let clone_config = config.clone();

    let app = gtk4::Application::builder().application_id(APP_ID).build();
    // let settings = Settings::new(APP_ID).set_value("conf", &config);
    app.connect_startup(move |_| ui::css::load_css(&clone_config));
    app.connect_activate(move |app| ui::build_ui(app, &config));
    app.run_with_args(&Vec::<String>::new())
}
