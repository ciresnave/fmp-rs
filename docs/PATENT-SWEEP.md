# Patent sweep: fmp-rs 0.1.7 (library code swept @ dd1b628; src/ unchanged on main through 91c94e0) - PARTIAL

Date: 2026-10-25; Run by: fmp-rs lane, Claude Sonnet 5.5; Method: AI-assisted search and read; NOT legal advice; NOT a freedom-to-operate opinion.

**Status: PARTIAL.** Idea 3 (rate limiting / retry) is best covered. A first run was cut short by HTTP 503 from Google Patents
(our own query burst); a second run on 2026-10-25 (paced, one query per minute) ran the five missing queries and read the UNREAD
rows. Ideas 1, 2, 4, 5 are still thinly searched (see "Coverage gaps"); idea 5 got one query in a third run (hits judged by title only).

## Ideas swept (fmp-rs is a REST client; little is novel)
1. Response cache whose TTL is chosen by substring-matching the endpoint path (`SmartCacheStrategy::get_ttl_for_endpoint`, src/cache.rs).
2. Per-key TTL cache with a max-items bound and hit/miss/expiry metrics (`IntelligentCache`, src/cache.rs).
3. Client-side request spacing at a fixed minimum interval (`apply_rate_limiting`) plus retry on 429/5xx/timeouts with exponential backoff and 0-100 ms random jitter (`ProductionExecutor`, src/production.rs). No `Retry-After` handling; no adaptation of the rate from outcomes.
4. Processing a large dataset in fixed-size chunks under a configured memory cap (`ChunkProcessor`, `StreamingProcessor`, src/bulk_processing.rs).
5. Skipping the cache for bulk/streaming endpoints by path substring (`should_cache_endpoint`).

## Queries (source: Google Patents xhr query, US granted + applications, priority after 2005, run 2026-10-25)
| idea | query | result |
|---|---|---|
| 1 | `("time to live" OR ttl) cache ("endpoint" OR "uniform resource locator" OR "url path") "api" client response (type OR category) determine` | 3619 results; top 12 are CDN/DNS caching (Cedexis, Amazon, Level 3, Dnsstuff); Amazon US10783077B2 and US10623408B1 read |
| 1 | `"time to live" cache "request" "path" pattern match "different" "time to live" "types of" data api response` | 1403; CDN family (Level 3, Intel IoT); none is client-side API-library caching; not read beyond title/snippet |
| 2 | `cache "time to live" "maximum number of entries" evict expired hit miss metrics` | 2 results (Amazon US12039358B1, packet flow); not read |
| 3 | `client "rate limit" retry "exponential backoff" jitter api request` | 4 results; US12141033B2 and US11243853B2 read |
| 3 | `"exponential backoff" "random" jitter retry delay request client` | 25; US9973306B2 read |
| 3 | `"retry" "status code" "429" OR "too many requests" client delay "retry-after" request` | 15; US11463362B2 and US20250337808A1 read |
| 3 | `"token bucket" OR "sliding window" "client" "rate limit" "outgoing requests" delay before sending api` | 0 results |
| 4 | `chunk "memory usage" limit dataset processing "chunk size" stream processor` | 112; Amazon ML-data family (US10318882B2 etc.), Acronis; not read |
| 4 | `bulk data "memory limit" "chunk" "process" "progress" streaming "batch size" large file records` | 0 results |
| 4 | general web search (earlier today) for adaptive chunking | US8707320B2 read; US8819288B2 found, not read |
| 1 | `cache expiration time determined by request type api client` (second run) | 48286; CDN/mobile-caching (Amazon, Seven Networks); Amazon US8412823B1 read |
| 2 | `cache "time to live" LRU eviction hit rate metrics maximum entries client` (second run) | 27; prefetch (Microsoft), flow management (Amazon), CDN (Level 3); none read |
| 4 | `chunk "memory threshold" process dataset in chunks adjust chunk size memory limit` (second run) | 139; Splunk/Pure Storage/Commvault; Software AG US10715448B2 read |
| 4 | `"memory budget" backpressure streaming records chunk processor` (second run) | 1 (OpenAI matrix-multiplication, irrelevant) |
| 3 | `"minimum interval" between successive requests client throttle api rate limit` (second run) | 18; Amazon streaming-data family; Kaseya US20250094286A1 (same family as row 3, not read); none read beyond title |
| 5 | `"bypass" OR "skip" cache "bulk" OR "streaming" request "uniform resource locator" OR "endpoint" client api response not cached` (third run) | 46695; Citrix/Plamondon proxy-cache families (freshness, prefetch, revalidation); none read beyond title; none about a client library skipping its cache by path |

## Hits
| # | patent | title | assignee | status (Google Patents, not the official record) | claims read | overlap | our difference | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | US8707320B2 | Dynamic partitioning of data by occasionally doubling data chunk size | Microsoft | Expired - Fee Related | independent 1, 8, 16 | chunks a stream (idea 4) | every independent claim needs a growing power-of-two chunk-size sequence assigned to concurrent threads; ours is one fixed configured size | clear |
| 2 | US11463362B2 | Dynamic token bucket method adaptive to opaque server limits | Netskope | Active; est. exp. 2041-01-29 | independent 1, 15, 18 | throttles API calls to a SaaS (idea 3) | all independents require intercepting requests AND responses of a user group, inferring time-varying server limits from event rate vs penalty responses, a dynamically set throttle limit, and an overflow event channel via a proxy's auxiliary queue; ours is a static minimum interval in one process | clear |
| 3 | US12141033B2 | Fleetwide adaptive rate limiting gatekeeper | Datto | Active; est. exp. 2042-12-16 | independent 1, 18, 19, 20 | retry-after; jitter (dep. 10); backoff (dep. 15) | independents require permission request/response data structures naming an API and a set of scopes, per-scope retry-after intervals, wait = max across scopes, and blocking clients until it passes; ours has no scopes, no permission protocol, and does not read `Retry-After` | clear |
| 4 | US11243853B2 | Service retry collision prevention using prime numbers | IBM | Expired - Fee Related | independent 1, 8, 14 | delays REST retries | every independent claim requires the retry delay to be a prime number of time units; ours is multiplier backoff plus 0-100 ms jitter with no prime constraint | clear |
| 5 | US20250337808A1 | Adaptive throttling of storage service traffic within a client application | Oracle | Pending (application) | independent 1, 8, 15 | "adaptively throttling requests ... based on success or failure rates of the monitored requests" | claims also require a computing cluster accessing a capacity-limited storage service shared by plural clients; ours spaces requests at a fixed interval and retries per request, and the `Metrics` counters are not fed back into throttling | clear on the current text; **the application is pending and broad: re-check at grant, and before any change that adapts the rate from outcomes** |
| 6 | US10783077B2 | Managing resources using resource expiration data | Amazon | Active; est. exp. 2029-07-10 | independent 1, 11 | TTL on cached resources (ideas 1, 2) | requires hierarchically arranged cache servers and updating expiration from request counts to the higher level; ours is one in-process cache with fixed TTLs | clear |
| 7 | US10623408B1 | Context sensitive object management | Amazon | Active | independent 1, 3, 10 | invalidates a cache entry before its TTL | requires a CDN provider, DNS-query requests, and per-user context criteria with request-count thresholds; ours has none | clear |
| 8 | US9973306B2 | Freshness-sensitive message delivery | Amazon | Active; est. exp. 2036-04-15 | independent 1, 5, 13 | exponential-backoff retry (dep. 2, 7) | event-stream message delivery with freshness values and non-blocking later events; ours retries a single HTTP request | clear |
| 9 | US8819288B2 | Optimized data stream compression using data-dependent chunking | Microsoft | Expired - Fee Related | independent 1, 10, 14 (claim 14 read in part) | chunks a data stream (idea 4) | every read independent requires partitioning by previously seen byte patterns, signatures matched against stored chunks, and forward/backward match-length scans for compression; ours is fixed-size chunking with no signatures or compression | clear (expired in any case) |
| 10 | US12039358B1 | Efficient management of packet flow information at network function virtualization services | Amazon | Active; est. exp. 2042-10-12 | independent 1 (6 read in part) | expiry-based eviction (idea 2) | requires a signature table of buckets of packet-flow entries on a network-function-virtualization service, a multi-element object indexed by bucket and entry identifiers, and eviction of a bucket's entries on a packet arrival; ours is an in-process key/TTL map | clear |
| 11 | US10318882B2 | Optimized training of linear machine learning models | Amazon | Active; est. exp. 2036-10-27 | independent 1, 6 | memory-driven pruning (idea 4) | requires an ML service pruning a parameter vector of model weights when a trigger fires, by quantile analysis without a sort; unrelated to our chunking. Sibling family members (US10339465B2, US9672474B2) NOT read: same ML-training subject, low relevance | clear (this member); siblings UNREAD |
| 12 | US10715448B2 | Predictive resource management in file transfer servers | Software AG, now IBM | Active | independent 1, 8, 14, 15, 17 | throttles requests by a "speed limiter" (idea 3) | every independent requires two machine-learned models (time-dependent usage and file data), plural predictor processes, and an accept/reject decision per incoming request on the server side; ours is a client-side fixed interval with no models | clear |
| 13 | US8412823B1 | Managing tracking information entries in resource cache components | Amazon | Expired - Fee Related | independent 1, 11, 18 | invalidates cache before expiry (idea 2) | requires a content delivery service provider and invalidation of tracking-information entries from an origin source; ours has neither | clear (expired in any case) |
| 14 | US20250094286A1 | Fleetwide adaptive rate limiting gatekeeper (continuation of row 3) | Kaseya | Pending (application); est. exp. 2042-12-16 | independent 1, 8, 14 | retry-after; throttled-response handling (idea 3) | claims 1 and 14 need API-call-permission data structures naming an API and scopes, per-scope retry-after, wait = max across scopes, and blocking clients; claim 8 needs accessor usage profiles (time period, constraint, accessor type) in a multi-tenant infrastructure; ours has none of these and ignores `Retry-After` | clear on the current text; pending: re-check at grant |

## Coverage gaps (the reason this is PARTIAL)
- Ideas 1, 2, 4: each now has 3-4 queries, but most top hits were judged by title/snippet only (CDN, Splunk, Level 3, Seven Networks families); only the closest were read.
- Idea 5 (skip cache for bulk/streaming paths): one query; top hits (Citrix proxy caches) judged by title only.
- Idea 3: the `token bucket` query returned 0 results. The US10339465B2/US9672474B2 siblings (ML-training subject, low relevance) are UNREAD.
- US only; no EP / WIPO search.
- Next: a second idea-5 query and read its closest hits, then drop the PARTIAL label only if the PM accepts the remaining gaps.

## Needs review (to the PM)
No overlap found on any independent claim read. Row 5 (Oracle, pending) is the one to watch. Nothing is `needs-review` today,
but the file is still PARTIAL: do not treat it as covering a release until the gaps above are closed or the PM accepts them.

## Limitations
AI-assisted; Google Patents text is machine-converted; status and expiry are Google Patents estimates, not the official register; US only; search coverage incomplete as stated; not legal advice.
