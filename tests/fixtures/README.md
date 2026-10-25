# Test fixtures

Responses served by wiremock in `tests/cot.rs`. Files marked **synthetic** were
written by hand to exercise the parsing and the request shape; they are NOT
recorded FMP output and prove nothing about FMP's real column names. Replace them
with recorded responses (key supplied via the vault, never committed) and drop
the "synthetic" mark.

## Recording a fixture

With the key supplied by the vault tooling as `FMP_API_KEY` (never a file here):

    cargo run --example record_fixture -- /commitment-of-traders-list tests/fixtures/cot_list.json

The tool writes the response body only, refuses non-2xx or non-JSON bodies, and aborts
if the body contains the key. Review the file before committing (no personal data), rename
away `.synthetic`, and point the test at the recorded file. The failure paths (no key,
rejected key) are exercised; the success path has not run without a key as of 2026-10-25.
