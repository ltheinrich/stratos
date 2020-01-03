//! Stratos command-line interface

use kern::cli::Command;
use std::{
    env,
    fs::File,
    io::prelude::{Read, Write},
};
use stratos::*;

const HELP: &str = "Benutzung: scli LOG-DATEI [OPTIONEN]\nString S, Float F, Integer I, Erforderlich *\n\nOptionen:
* --x    S       x-Achse
* --y    S       y-Achse
  --o    S       Ausgabe-Datei
  --s    I       Punkt-Größe
  --h    S       Höhe im Log (Aufstieg-Farbe Standard, Abstieg-Farbe --cf)
  --c    S       HTML-Farbcode
  --cf   S       HTML-Farbcode für den Abstieg (benötigt --h)
  --xn   S       x-Achsen-Name
  --yn   S       y-Achsen-Name
  --xmin F       Minimum x-Bereich
  --xmax F       Maximum x-Bereich
  --ymin F       Minimum y-Bereich
  --ymax F       Maximum y-Bereich";

fn main() {
    // init
    init_version();
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
    let x_min = cmd.get_parameter("xmin");
    let x_max = cmd.get_parameter("xmax");
    let y_min = cmd.get_parameter("ymin");
    let y_max = cmd.get_parameter("ymax");
    let size = cmd.get_parameter("s");
    let colour = cmd.get_parameter("c");
    let colour_fall = cmd.get_parameter("cf");
    let height = cmd.get_parameter("h");

    // read log file
    let mut file = String::new();
    File::open(path)
        .expect("Log-Datei konnte nicht geöffnet werden")
        .read_to_string(&mut file)
        .expect("Log-Datei konnte nicht gelesen werden");

    let analysis = draw(
        &file,
        Parameters::from(
            x_axis,
            y_axis,
            x_name,
            y_name,
            x_min,
            x_max,
            y_min,
            y_max,
            size,
            colour,
            colour_fall,
            height,
        ),
    )
    .unwrap();

    // save output file
    File::create(output)
        .expect("Die Ausgabe-Datei kann nicht erstellt werden")
        .write_all(analysis.as_bytes())
        .expect("Die Ausgabe-Datei kann nicht geschrieben werden");
    println!("Analyse gespeichert: {}", output);
}
