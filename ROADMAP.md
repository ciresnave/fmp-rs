# fmp-rs ROADMAP

| Row | Why | Exit / deadline |
|---|---|---|
| Leave chrono | RUSTSEC-2026-0543 (chrono unmaintained, no safe upgrade) is ignored in `deny.toml` until 2026-12-17 | Move timestamps to jiff or time; drop the `deny.toml` ignore when chrono is out of Cargo.lock |
