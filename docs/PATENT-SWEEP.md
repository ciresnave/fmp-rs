# Patent sweep: fmp-rs 0.1.3 @ a6b2f36 (main) - INCOMPLETE DRAFT

Date: 2026-10-25; Run by: fmp-rs lane, Claude Sonnet 5.5; Method: AI-assisted search and read; NOT legal advice; NOT a freedom-to-operate opinion.

**Status: INCOMPLETE. Not sufficient for a release.** The search tool available to this lane (web search) returned
documentation and articles, not patent records, for ideas 1-3 and 5; only one patent was located and read in full. Ideas
without a read hit are `unread`, never `clear`. A real sweep needs Google Patents / USPTO / Espacenet queries run by a
session with direct patent-database access.

## Ideas swept (what the crate does that might be unusual; it is a REST client, so little is)
1. Response cache whose TTL is chosen by substring-matching the endpoint path (`SmartCacheStrategy::get_ttl_for_endpoint`, src/cache.rs).
2. Per-key TTL cache with a max-items bound and hit/miss/expiry metrics (`IntelligentCache`).
3. Client-side request rate limiter combined with retry using exponential backoff plus random jitter (`ProductionExecutor`, src/production.rs).
4. Processing a large dataset in fixed-size chunks under a memory-usage cap (`ChunkProcessor`, `StreamingProcessor`, src/bulk_processing.rs).
5. Opt-out of caching for bulk/streaming endpoints by path substring (`should_cache_endpoint`).

## Queries (source, query, date)
| source | query | date | result |
|---|---|---|---|
| web search | patent cache time-to-live determined by request URL path endpoint type API client response caching | 2026-10-25 | no patent records; vendor docs only (path-based TTL is common product practice) |
| web search | patent exponential backoff retry with random jitter client rate limiting API requests | 2026-10-25 | no patent records; articles only |
| web search | patent processing large dataset in chunks memory budget limit streaming processor chunk size adaptive | 2026-10-25 | two patent records (below) |

## Hits
| # | patent | title | assignee | status | claim(s) read | overlap | our difference | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | US8707320B2 | Dynamic partitioning of data by occasionally doubling data chunk size for data-parallel applications | Microsoft | Expired - Fee Related (Google Patents, not the official record) | independent 1, 8, 16, all read in full via tools/patent-claims.py | chunks a stream | every independent claim requires a growing chunk-size sequence (repeated powers of two with doubling) assigned to concurrent threads on a multi-node system; ours uses one fixed configured chunk size and no such sequence | clear (idea 4) |
| 2 | US8819288B2 | Optimized data stream compression using data-dependent chunking | (not recorded) | (not recorded) | NOT READ | - | - | unread |

## Needs review (to the PM)
Nothing found that overlaps. But ideas 1, 2, 3, 5 have NO patent read, and hit 2 is unread. Do not treat this file as covering a release;
re-run with a patent-database-capable session, then replace this draft.

## Limitations
AI-assisted; web-search coverage only; most queries returned no patent documents; claim text is machine-converted from Google Patents;
status from Google Patents, not an official register; not legal advice.
