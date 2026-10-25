# Test fixtures

Responses served by wiremock in `tests/cot.rs`. Files marked **synthetic** were
written by hand to exercise the parsing and the request shape; they are NOT
recorded FMP output and prove nothing about FMP's real column names. Replace them
with recorded responses (key supplied via the vault, never committed) and drop
the "synthetic" mark.
