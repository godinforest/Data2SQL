# Data2SQL

A fast, desktop-based Extract, Transform, and Load (ETL) application built with Rust and Iced. This app allows you to drag and drop structured and semi-structured data files to automatically convert and load them into SQLite databases. Don't spend your tokens! Just drag, drop and your NoSQL data now SQL-ready.

## Features

* **Drag & Drop Interface:** Add files to the processing queue simply by dropping them into the application window.
* **Multiple Formats:** Built-in support for JSON, CSV, TSV, and XML extraction.
* **Asynchronous Processing:** Powered by `tokio`, preventing UI freezes during heavy I/O operations and database transactions.
* **Optimized Loading:** Automatically batches parsed records to maximize SQLite write performance.
* **Modern UI:** Split-screen interface with dynamic queue status and export history tracking.

## Prerequisites

Before building or running the project, ensure you have the following installed on your system:

1.  **Rust & Cargo:** Install the latest stable toolchain via [rustup](https://rustup.rs/).
2.  **C Compiler:** Required to compile the bundled `rusqlite` dependency.
    * *Windows:* Visual Studio Build Tools (C++ workload).
    * *macOS:* Xcode Command Line Tools (`xcode-select --install`).
    * *Linux:* GCC or Clang (`sudo apt install build-essential`).

## How to Run

You can build and launch the application directly from your terminal.

1. Clone the repository and navigate to the project directory:
```bash
git clone https://github.com/godinforest/Data2SQL.git
cd data_pump
```

Run the application in development mode (useful for debugging, but processes data slower):
```bash
cargo run
```
Recommended for large files: Run the application in release mode. This enables compiler optimizations, making data parsing and SQLite insertions significantly faster:
```bash
cargo run --release
```


Launch the application.

On the right panel, click Choose directory to select the output folder for the generated .db files.

Drag and drop .json, .csv, .tsv, or .xml files into the left panel.

The processing will start automatically. You can track the progress in the queue.

Once a file is processed, click Show Folder to access the resulting SQLite database.
