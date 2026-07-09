data_pump/
├── Cargo.toml               # Project metadata and dependencies (iced, serde_json, csv, quick-xml, rusqlite)
├── README.md                # Project documentation
└── src/
    ├── main.rs              # Application entry point, logger initialization, and iced::Application launch
    ├── app.rs               # Main DataPumpApp structure and trait implementations
    │
    ├── ui/                  # UI LAYER (iced Retained Mode)
    │   ├── mod.rs           # UI module declarations
    │   ├── messages.rs      # Message enum for all user and system interactions (FileDropped, CancelPressed)
    │   ├── state.rs         # UI state definitions (ImportQueue, ExportHistory, UI specific buffers)
    │   ├── theme.rs         # Custom styling, colors, and layout constraints
    │   └── view/            # Rendering logic returning iced::Element
    │       ├── mod.rs       # View module declarations
    │       ├── import_zone.rs # Drag & drop area, format indicators, import queue rendering
    │       ├── export_zone.rs # Output directory selector, export history, failure logs
    │       └── modals.rs    # Overlay windows (e.g., manual extension override selector)
    │
    └── core/                # BUSINESS LOGIC LAYER (ETL Engine)
        ├── mod.rs           # Core module declarations
        ├── task.rs          # Asynchronous workers (iced::Command/Subscription) for non-blocking UI
        ├── telemetry.rs     # User journey and error tracking logging to local .jsonl file
        │
        ├── extract/         # EXTRACT LAYER (File parsing)
        │   ├── mod.rs       # Extract module declarations
        │   ├── factory.rs   # Extension detection, reader instantiation, FIFO queue handling
        │   ├── json.rs      # JSON parser (yields records)
        │   ├── csv.rs       # CSV and TSV parser (yields records)
        │   └── xml.rs       # XML streaming parser (yields records)
        │
        ├── transform/       # TRANSFORM LAYER (Data mapping)
        │   ├── mod.rs       # Transform module declarations
        │   └── mapper.rs    # Data validation and type mapping before database insertion
        │
        └── load/            # LOAD LAYER (Database operations)
            ├── mod.rs       # Load module declarations
            └── sqlite.rs    # SQLite transactions, handling of "NotFinished_" temporary files