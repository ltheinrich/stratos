[![Build Status](https://ltheinrich.de/stratos/workflows/CI/badge.svg)](https://ltheinrich.de/stratos/actions?query=workflow%3ACI)

# Stratos
### Stratosphärenflug Log-Analyse
Stratos analysiert den Log vom Stratosphärenflug.
Dabei werden ganze und Dezimalzahlen unterstützt. Zeitangaben (hh:mm:ss) werden in Minuten umgerechnet.

## Kompilieren
Anforderungen
 - Git
 - Rust
 - Cargo

Git-Repository klonen
> git clone https://github.com/ltheinrich/stratos && cd stratos

Mit Cargo kompilieren
> cargo build --release

Die ausführbaren Datei (stratos unter Linux bzw. stratos.exe unter Windows) befindet sich unter `target/release/`
