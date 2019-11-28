extern crate kern;

mod parse;

use kern::{cli::Command, meta::version};
use plotlib::{
    page::Page,
    scatter::{Scatter, Style},
    style::Point,
    view::ContinuousView,
};
use std::{env, fs::File, io::prelude::Read};

const CARGO_TOML: &str = include_str!("../Cargo.toml");
const HELP: &str = "Benutzung: stratos LOG-DATEI [OPTIONEN]\nString S, Float F, Integer I, Erforderlich *\n\nOptionen:
* --x    S       x-Achse
* --y    S       y-Achse
  --o    S       Ausgabe-Datei
  --s    I       Punkt-Größe
  --c    S       HTML-Farbcode
  --xn   S       x-Achsen-Name
  --yn   S       y-Achsen-Name
  --xmin F       Minimum x-Bereich
  --xmax F       Maximum x-Bereich
  --ymin F       Minimum y-Bereich
  --ymax F       Maximum y-Bereich";

fn main() {
    // init
    println!(
        "Stratos {} (c) 2019 Lennart Heinrich\n",
        version(CARGO_TOML)
    );
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        println!("{}", HELP);
    }

    // parse command line
    let cmd = Command::from(&args, &[]);
    let path = cmd.get_argument(0).expect(HELP);
    let x_axis = cmd.get_parameter("x").expect(HELP);
    let y_axis = cmd.get_parameter("y").expect(HELP);
    let output = cmd.get_parameter("o").unwrap_or(&"analyse.svg");
    let x_name = cmd.get_parameter("xn");
    let y_name = cmd.get_parameter("yn");
    let x_min = cmd.get_parameter("x_min");
    let x_max = cmd.get_parameter("x_max");
    let y_min = cmd.get_parameter("y_min");
    let y_max = cmd.get_parameter("y_max");
    let size: u8 = cmd.get_parameter("s").unwrap_or(&"1").parse().expect(HELP);
    let colour = cmd.get_parameter("c").unwrap_or(&"#DD3355");

    // read log file
    let mut file = String::new();
    File::open(path)
        .expect("Log-Datei konnte nicht geöffnet werden")
        .read_to_string(&mut file)
        .expect("Log-Datei konnte nicht gelesen werden");

    // parse log file
    let log = parse::parse_log(&file);
    let x_values = log
        .at_key(x_axis)
        .expect("Die Werte der x-Achse sind fehlerhaft");
    let x_values = parse::into_f64(&x_values);
    let y_values = log
        .at_key(y_axis)
        .expect("Die Werte der y-Achse sind fehlerhaft");
    let y_values = parse::into_f64(&y_values);

    // create scatter
    let values = parse::into_tuple(x_values, y_values);
    let scatter =
        Scatter::from_slice(&values).style(Style::new().colour(colour.to_string()).size(size));
    let mut view = ContinuousView::new()
        .add(&scatter)
        .x_label(if let Some(x_name) = x_name {
            x_name.to_string()
        } else {
            x_axis.to_string()
        })
        .y_label(if let Some(y_name) = y_name {
            y_name.to_string()
        } else {
            y_axis.to_string()
        });

    // add x/y range
    if let (Some(x_min), Some(x_max)) = (x_min, x_max) {
        view = view.x_range(x_min.parse().expect(HELP), x_max.parse().expect(HELP))
    }
    if let (Some(y_min), Some(y_max)) = (y_min, y_max) {
        view = view.y_range(y_min.parse().expect(HELP), y_max.parse().expect(HELP))
    }

    // save output file
    Page::single(&view)
        .save(output)
        .expect("Die Analyse konnte nicht abgespeichert werden");
    println!("Analyse gespeichert: {}", output);
}
