extern crate kern;

mod parse;
mod plot;

use kern::cli::Command;
use kern::meta::version;
use std::env;

static CARGO_TOML: &str = include_str!("../Cargo.toml");

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::from(&args, &[""]);

    if command.is_option("unimplemented") {
        unimplemented!();
    } else {
        println!("Benutzung: strato MODUS [OPTIONEN]");
        println!();
        println!("Modus:");
        println!("  web             -/Webinterface starten");
        println!("  analyse         Logdatei analysieren");
        println!();
        println!("Optionen:");
        println!("  --port  N       Webinterface Port (8080)");
        println!("  --limit N       Webinterface maximale Logdatei-Größe in kB (5000)");
        println!("  --datei S       Logdatei-Pfad");
        println!();
        println!("Strato {} (c) 2019 Lennart Heinrich", version(CARGO_TOML));
        use std::io::prelude::Read;
        let mut buf = String::new();
        std::fs::File::open("feb19.log")
            .unwrap()
            .read_to_string(&mut buf);
        let log = parse::parse_log(&buf);
        let alt = parse::into_f64(&log.at_key("Altitude NN [m]").unwrap());
        let temp = parse::into_f64(&log.at_key("Extern: Temp [C]").unwrap());
        use plotlib::{
            page::Page,
            scatter::{Scatter, Style},
            style::{Marker, Point},
            view::ContinuousView,
        };
        let mut l = Scatter::from_slice(&parse::into_tuple(temp, alt))
            .style(Style::new().marker(Marker::Circle).colour("#DD3355"));
        let v = ContinuousView::new()
            .add(&l)
            .x_label("Außentemperatur in °C")
            .y_label("Höhe in Meter");
        Page::single(&v).save("scatter.svg").unwrap();
    }
}

// plot
/*
use plotlib::{
    page::Page,
    scatter::{Scatter, Style},
    style::{Marker, Point},
    view::ContinuousView,
};

let mut l = Scatter::from_slice(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
    .style(Style::new().marker(Marker::Cross).colour("#DD3355"));
let v = ContinuousView::new().add(&l).x_label("One").y_label("Two");

Page::single(&v).save("scatter.svg").unwrap();
*/
