//! Constant variables

use kern::meta;

pub const HEAD: &str = include_str!("../../web/head.html");
pub const BACK: &str =
    "<a href=\"/\"><button class=\"btn btn-outline-secondary btn-sm\">Zurück</button></a>";
pub const INDEX: &str = include_str!("../../web/index.html");
pub const OPTIONS: &str = include_str!("../../web/options.html");
pub const BOOTSTRAP: &[u8] = include_bytes!("../../web/bootstrap.min.css");
pub const STYLE: &[u8] = include_bytes!("../../web/style.css");
pub const FAVICON_ICO: &[u8] = include_bytes!("../../web/favicon.ico");
pub const FAVICON_PNG: &[u8] = include_bytes!("../../web/favicon.png");
pub const APPLE_TOUCH_ICON: &[u8] = include_bytes!("../../web/apple-touch-icon.png");
pub const HELP: &str = "Benutzung: stratos [OPTIONEN]\nString S, Integer I, Boolean B\n\nOptionen:
  --port    I       Port (3490)
  --addr    S       IP-Adresse ([::])
  --size    I       Maximale Log-Größe in MB (10)
  --threads I       Anzahl der anfangs startenden Threads (2)
  --log             Logging der Anfragen aktivieren";
const CARGO_TOML: &str = include_str!("../../Cargo.toml");
static mut VERSION: &str = "";

/// Get HTML footer with version
pub fn footer() -> String {
    format!("</div></div><div class=\"cr\"><small class=\"form-text text-muted\">Stratos v{} &copy; 2019 Lennart Heinrich</small><a href=\"https://github.com/ltheinrich/stratos/issues\">Fehler melden</a></div></body></html>", version())
}

/// Get version
pub fn version() -> &'static str {
    unsafe { VERSION }
}

/// Set version (unsafe!)
pub fn init_version() {
    unsafe {
        VERSION = meta::version(CARGO_TOML);
        println!("Stratos {} (c) 2019 Lennart Heinrich\n", VERSION);
    }
}
