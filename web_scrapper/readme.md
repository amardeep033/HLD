# Web Scraper & Web Crawler — Deep Study Notes

Story-driven notes covering the full arc: why crawlers exist, how they're designed at scale, and how to implement them in Rust.

---

## Table of Contents

1. [The Problem Before It Existed](#1-the-problem-before-it-existed)
2. [Why This Thing Was Invented](#2-why-this-thing-was-invented)
3. [The Big Picture — Where It Fits](#3-the-big-picture--where-it-fits)
4. [What They Actually Do](#4-what-they-actually-do)
5. [When You Need It vs When You Don't](#5-when-you-need-it-vs-when-you-dont)
6. [Clear Distinctions — Scraper vs Crawler](#6-clear-distinctions--scraper-vs-crawler)
7. [The Implementation](#7-the-implementation)

---

## 1. The Problem Before It Existed

### 1.1 The Pain

The early web had no memory.

- A new page published anywhere on the internet was invisible to everyone unless someone *already knew the URL*.
- Search engines could not find pages they hadn't been told about.
- The web was growing faster than any team of humans could manually catalog.
- Data on websites — prices, scores, job listings — changed hourly. No system could track that without re-visiting every page by hand.

### 1.2 What Was Actually Breaking

- **Google's index went stale.** If a page updated, no one knew unless the URL was manually re-submitted.
- **Price comparison was impossible.** You couldn't aggregate product prices across 500 sites without visiting all 500 manually.
- **Financial intelligence was locked.** Firms wanted to mine annual reports and shareholder filings sitting on thousands of company websites — no tool existed to do this automatically.
- **Copyright violations went undetected.** Pirated content could spread across the web with no mechanism to find it.

> The core pain: **the internet was write-only from a discovery perspective.** Publishing was easy. Finding, monitoring, and aggregating was broken.

---

## 2. Why This Thing Was Invented

### 2.1 What Specific Problem It Solves

Two tools were invented to fix two distinct problems:

- **Web Scraper** — given a page you already have, extract the structured data out of it.
- **Web Crawler** — automatically discover new pages you don't know about yet, by following links.

### 2.2 What Would Go Wrong Without Them

- Without scrapers: every data extraction job requires a human to copy-paste from a browser. No automation. No pipelines. No monitoring.
- Without crawlers: search engines cannot index the web. You cannot build a sitemap. You cannot archive the internet. Google does not exist.

### 2.3 Real Use Cases That Make It Concrete

| Use Case | Tool | What It Does |
|---|---|---|
| Google Search | Crawler | Discovers and indexes the entire public web |
| US Library of Congress | Crawler | Archives websites before they go offline |
| Price comparison site | Scraper | Pulls product prices from 500 retailer pages |
| Financial intelligence firm | Scraper + Crawler | Downloads annual reports from thousands of company sites |
| Copyright monitoring (Digimarc) | Crawler | Detects pirated works across the internet |
| Cricket score tracker | Scraper | Extracts live scores from a sports page |

---

## 3. The Big Picture — Where It Fits

### 3.1 Mental Map

Think of the internet as a **directed graph**:

```text
Web Pages  = nodes
Hyperlinks = edges (directed — one page links to another)
```

A **web crawler is just graph traversal on this giant graph** — BFS or DFS, at internet scale.

You already know:
- `HashMap<URL, visited>` → that's your visited set
- `VecDeque<URL>` → that's your URL frontier (BFS queue)
- `HashSet<URL>` → deduplication

The crawler is just those data structures running across HTTP.

### 3.2 How It Relates to Things You Already Know

| Familiar Concept | Web Crawling Equivalent |
|---|---|
| Graph BFS | Crawling level by level from seed URLs |
| Directory scanner that follows symlinks | Crawler following links across domains |
| `HashMap<key, seen>` | `URL Seen?` component |
| File download manager | HTML Downloader |
| Content hash check | `Content Seen?` deduplication |

### 3.3 Scale That Changes Everything

This isn't a toy — the numbers are what make the design non-trivial:

```text
1 billion pages/month
→ QPS = 1,000,000,000 / 30 / 24 / 3600 ≈ 400 pages/sec
→ Peak QPS ≈ 800

Average page size: 500 KB
→ 1B pages × 500 KB = 500 TB/month
→ 5 years of storage = 500 TB × 12 × 5 = 30 PB
```

At this scale, every naive implementation breaks. That's the entire reason the design below exists.

---

## 4. What They Actually Do

**Web Scraper** — visits a page you point it at, extracts structured data from it, and gives you back clean records.

**Web Crawler** — starts from a set of seed URLs, fetches each page, pulls out all links, and repeats recursively — discovering the web graph on its own.

> One sentence: a scraper **extracts**, a crawler **discovers**.

---

## 5. When You Need It vs When You Don't

### 5.1 You Reach for a Scraper When...

- The site has no API and you need data from it.
- You need to monitor a page for changes (prices, scores, listings).
- You're extracting a one-time dataset from a small number of known URLs.
- An SDE2 discussion asks: *"Extract all product titles from this page."*

### 5.2 You Reach for a Crawler When...

- You don't know all the URLs upfront — you need to discover them.
- You're building a search index and need to map an entire domain.
- You're archiving a site before it goes offline.
- An SDE2 discussion asks: *"Find all reachable links on this website."*

### 5.3 You Don't Need Either When...

- The site has a public API — use the API.
- You only need a single known URL with no link-following.
- The data is available via RSS, sitemap.xml, or a data export.

---

## 6. Clear Distinctions — Scraper vs Crawler

People confuse these constantly. The confusion comes from the fact that crawlers often *contain* scrapers — but they are not the same thing.

| Feature | Web Scraper | Web Crawler |
|---|---|---|
| Goal | Extract data from known pages | Discover unknown pages |
| Traversal | Single page or a fixed small set | Recursive, follows links indefinitely |
| Data extraction | Core purpose | Optional — can happen alongside crawling |
| BFS / DFS | Rare | Core requirement |
| Visited URL tracking | Rare | Required — without it you loop forever |
| Queue management | Rare | Required |
| Rate limiting / politeness | Optional | Critical |
| Scaling complexity | Medium | High |

> **Key mental model:** A scraper is a knife — sharp and focused on one target. A crawler is a fishing net — thrown wide, pulls back everything it touches.

### 6.1 The Nested Reality

```text
Web Crawler
├── URL Frontier (BFS queue)
├── HTML Downloader
└── Web Scraper ← embedded inside
    ├── Content Parser
    └── Link Extractor
```

A crawler uses a scraper internally to extract *links* from each page it visits. The scraper is a component of the crawler, not a synonym.

---

## 7. The Implementation

You now understand why. Here's how.

### Typical Crawler Architecture — The Mental Map

```text
Seed URLs
    → URL Frontier
    → Downloader Workers
    → HTML Parser
    → Link Extractor
    → URL Deduplication
    → Frontier Requeue  (loop)
```

Memorise this flow. Every component in the detailed design below maps to one of these stages.

### 7.1 The High-Level Architecture

The full system, in data-flow order:

```text
Seed URLs
    ↓
URL Frontier        ← BFS queue; stores URLs to be visited
    ↓
HTML Downloader     ← fetches raw HTML via HTTP
    ↓
DNS Resolver        ← URL → IP address (cached to avoid 10–200ms penalty per lookup)
    ↓
Content Parser      ← validates HTML; rejects malformed pages early
    ↓
Content Seen?       ← hash the page body; discard if duplicate (29% of web is dupes)
    ↓
Link Extractor      ← find all <a href="..."> tags, convert relative → absolute URLs
    ↓
URL Filter          ← drop blacklisted domains, invalid extensions, error links
    ↓
URL Seen?           ← bloom filter / hash table; skip already-visited URLs
    ↓
→ New URLs feed back into URL Frontier
```

> **Bootstrap alternatives:** Crawlers don't always start blind. `sitemap.xml` and RSS feeds are structured URL lists that sites publish explicitly — feeding these directly into the URL Frontier is faster and more polite than discovering everything by brute-force link-following. Always check for `sitemap.xml` before designing a full recursive crawl.

### 7.2 The Step-by-Step Workflow

```text
Step 1:  Seed URLs → URL Frontier
Step 2:  HTML Downloader pulls from URL Frontier
Step 3:  DNS Resolver: URL → IP (check cache first)
Step 4:  Download HTML page
Step 5:  Content Parser validates: malformed? → discard
Step 6:  Content Seen? hashes body
         → Hash found in storage? → discard (duplicate)
         → Hash not found? → pass to Link Extractor
Step 7:  Link Extractor pulls all <a href> links
Step 8:  URL Filter drops junk (blacklist, wrong file type, spider traps)
Step 9:  URL Seen? checks if already crawled
         → Seen? → skip
         → Not seen? → add to URL Frontier
Step 10: Loop
```

---

### 7.3 BFS vs DFS — The Traversal Decision

#### 7.3.1 Why DFS Fails

- DFS can go infinitely deep into one branch before coming up for air.
- Spider traps exploit this: `http://example.com/foo/bar/foo/bar/foo/bar/...`
- You never get breadth coverage — one domain eats all your crawl budget.

> **Note on DFS memory:** DFS does offer simpler recursive traversal and can use less memory on shallow graphs — but for web crawling this advantage disappears fast. Spider traps and unbounded depth make DFS dangerous in practice.

#### 7.3.2 Why BFS Wins

- Level-by-level traversal — you explore widely before going deep.
- Implemented with a FIFO queue (the URL Frontier).
- Naturally handles politeness when combined with per-host queues.
- **Broader coverage** — discovers more domains before diving deep into any one.
- **Avoids getting stuck** — no single branch consumes all crawl budget.
- **Better freshness distribution** — important pages close to seed URLs get crawled first.
- **Better parallelism** — wide frontier makes it easy to distribute work across many workers.

```text
BFS Traversal:
A
├── B
├── C
└── D

Order: A → B → C → D   ← breadth first

DFS Traversal:
Order: A → B → deeper into B's subtree first
```

> **Rule:** Web crawlers use BFS. DFS is for cases where depth matters more than breadth — almost never true for web crawling.

---

### 7.4 URL Frontier — The Heart of the System

The **URL Frontier** is the central data structure of a web crawler — the queue of URLs waiting to be downloaded.

**Responsibilities:**
- **Scheduling** — decide which URL gets fetched next
- **Prioritization** — fetch important pages first
- **Politeness** — prevent hammering a single host
- **Freshness** — recrawl pages that have changed

The URL Frontier is not just a queue. It solves two hard problems:

#### 7.4.1 Problem 1 — Politeness

**Politeness** is a first-class crawler design concern — discussioners expect you to name it explicitly.

**Politeness techniques:**
- Per-host queues — one queue per domain, one worker per queue
- Delays between requests to the same host
- Rate limiting per domain
- Concurrency caps per domain

Without control, a crawler hammers one server with thousands of requests per second. That's effectively a DoS attack.

**Solution: per-host FIFO queues (back queues)**

```text
Queue Router
    ├── b1  [wikipedia.com URLs only]  → Worker Thread 1
    ├── b2  [apple.com URLs only]      → Worker Thread 2
    └── bn  [nike.com URLs only]       → Worker Thread N

Mapping Table:
    wikipedia.com → b1
    apple.com     → b2
    nike.com      → bn
```

- One worker thread per host queue.
- One page at a time per host.
- Delay enforced between requests to the same host.

#### 7.4.2 Problem 2 — Priority

Not all pages are equal. The Apple homepage matters more than a random forum post about Apple products.

**Solution: priority queues (front queues)**

```text
Input URLs
    ↓
Prioritizer (scores by PageRank, traffic, update frequency)
    ↓
Front Queues [f1 (high priority), f2, ..., fn (low priority)]
    ↓
Queue Selector (biased random — picks high-priority queues more often)
    ↓
Back Queues (politeness layer)
    ↓
Worker Threads
```

> **Freshness scheduling:** Popular and high-value pages should be recrawled more frequently — not just crawled once. The priority system in the URL Frontier drives this. This is why freshness is a core Frontier responsibility, not an afterthought.

#### 7.4.3 Storage

- Pure memory = not durable (crash = lose the queue).
- Pure disk = too slow.
- **Solution: hybrid** — majority on disk, small in-memory buffers for enqueue/dequeue. Periodically flushed to disk.

> **Bounded queue:** An unbounded in-memory frontier will exhaust memory at scale. The in-memory buffer must be capped — overflow spills to disk immediately.

#### 7.4.4 Distributed Frontier

At large scale, a single machine cannot hold the entire URL Frontier.

- The frontier is **sharded across multiple machines** using consistent hashing on the URL or domain.
- Each shard manages an independent slice of the URL space.
- This directly bridges into distributed systems design — the same consistent hashing ring used in KV stores applies here.

---

### 7.5 Content Deduplication

#### 7.5.1 The Problem

29% of web pages are duplicate content under different URLs. Without deduplication, you store 30% more data than you need.

#### 7.5.2 The Solution

```text
Page downloaded
    ↓
Hash page body (not URL — two URLs can have identical content)
    ↓
Check hash against Content Storage
    ├── Found → discard page, continue crawling
    └── Not found → store page, extract links
```

**Content deduplication techniques:**
- **Hashing / checksums** — MD5 or SHA of page body; exact duplicate detection
- **Fingerprints** — SimHash or MinHash; detect near-duplicate pages with minor differences
- **Content similarity checks** — compare structural features when exact hash misses

URL deduplication (`URL Seen?`) uses a **bloom filter**:
- Fast, memory-efficient probabilistic check.
- **"Definitely not present"** — if the filter says absent, the URL is definitely not in the set. No false negatives.
- **"Possibly present"** — if the filter says present, the URL may or may not actually be in the set. False positives are possible but acceptable.

---

### 7.6 Robustness

#### 7.6.1 What Can Kill a Crawler

- Bad HTML that panics the parser.
- Unresponsive servers that block worker threads forever.
- Spider traps that generate infinite URL sequences.
- Crashes mid-crawl losing all state.

#### 7.6.2 Defenses

| Problem | Defense |
|---|---|
| Uneven load across downloaders | Consistent hashing to distribute URL space |
| Crash mid-crawl | Persist crawl state to storage; resume from checkpoint |
| Bad HTML | Validate in Content Parser; discard malformed pages |
| Spider traps | Cap max URL length; detect domains generating anomalous URL volume |
| Slow servers | Short timeout — skip and move on |
| Transient failures (timeouts, 5xx) | Retry with **exponential backoff** — avoids retry storms that amplify the original problem |
| DNS bottleneck | Cache DNS responses (DNS lookup = 10–200ms, synchronous) |

---

### 7.6.3 Spider Traps — A Classic Crawler Problem

A **spider trap** is a page (or chain of pages) that causes a crawler to loop infinitely, consuming resources without making progress.

**Examples:**
- Infinite calendar pages: `/calendar/2026/01/next → /calendar/2026/02/next → ...`
- Recursive URL patterns: `http://example.com/a/b/a/b/a/b/...`
- Generated query parameters: `?page=1 → ?page=2 → ?page=99999`

**Solutions:**

| Defense | How it helps |
|---|---|
| Max URL length | Cuts off impossibly long generated URLs |
| Max crawl depth | Stops going deeper after N hops from seed |
| Domain URL count limit | Flag domains generating anomalous URL volume |
| Heuristics | Detect repetitive path segments in URL structure |

> **discussion trigger:** Any time the discussioner asks *"what could go wrong with your crawler?"* — say **spider traps** first.

---

### 7.7 The Rust Stack

```text
reqwest   — async HTTP client (fetch raw HTML)
scraper   — HTML parser + CSS selector engine (extract data)
tokio     — async runtime (concurrent page fetching)
url       — URL parsing, relative → absolute resolution
```

---

### 7.8 HTML Basics — The DOM Model

```html
<a href="/careers">Careers</a>
```

Becomes:

```text
Element
├── tag    = a
├── attrs  = { href: "/careers" }
└── children = [ Text("Careers") ]
```

| Operation | Rust API | Returns |
|---|---|---|
| Get attribute | `.attr("href")` | `Option<&str>` |
| Get visible text | `.text()` | iterator over text nodes |

---

### 7.9 Core Scraper APIs

**7.9.1 Parse HTML**

```rust
let document = Html::parse_document(&html);
```

**7.9.2 Create CSS selector**

```rust
let selector = Selector::parse("a").unwrap();
```

**7.9.3 Traverse matching elements**

```rust
for element in document.select(&selector) {
    // element: ElementRef
}
```

**7.9.4 Get attribute**

```rust
let href = element.value().attr("href"); // Option<&str>
```

**7.9.5 Get text content**

```rust
let text = element
    .text()
    .collect::<Vec<_>>()
    .join(" ");
```

---

### 7.10 Link Extraction — The Core Problem

```text
URL
 ↓
Fetch HTML (reqwest)
 ↓
Parse DOM (Html::parse_document)
 ↓
Find <a> tags (Selector::parse("a"))
 ↓
Extract href (.attr("href"))
 ↓
Filter invalid links (#, javascript:void(0), mailto:...)
 ↓
Resolve relative → absolute URLs (base.join(relative))
 ↓
Return link list
```

**Links to filter out:**

```text
#                    ← fragment only, no page
javascript:void(0)   ← JS handler, not a real URL
mailto:user@mail.com ← email link
```

**Relative → absolute resolution:**

```rust
use url::Url;

let base = Url::parse("https://example.com").unwrap();
let full = base.join("/careers").unwrap();
// → "https://example.com/careers"
```

#### 7.10.1 URL Canonicalization

Different URLs can point to the same page. A crawler must normalize URLs before deduplication, or it will crawl the same content multiple times.

**Examples of the same page under different URLs:**

```text
https://site.com/page
https://site.com/page/           ← trailing slash variant
https://site.com/page#section    ← fragment — client-side only; server returns same document
https://site.com/page?utm=email  ← tracking param — same content, different URL
```

**Canonicalization steps:**
- Strip URL fragments (`#...`) — the server returns the same page regardless
- Normalize trailing slashes consistently
- Sort / strip known tracking query parameters (`utm_source`, `ref`, `fbclid`, etc.)
- Lowercase scheme and host
- Resolve relative URLs to absolute before storing

---

### 7.11 Structured Link Extraction — Full Example

```rust
use scraper::{Html, Selector};

struct Link {
    href: String,
    text: String,
}

fn extract_links(html: &str) -> Vec<Link> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();

    document
        .select(&selector)
        .filter_map(|el| {
            let href = el.value().attr("href")?.to_string();
            let text = el.text().collect::<Vec<_>>().join(" ");
            Some(Link { href, text })
        })
        .collect()
}
```

---

### 7.12 Basic Web Scraper — Fetch + Extract

```rust
use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let url = "https://example.com";
    let html = reqwest::get(url)
        .await.unwrap()
        .text()
        .await.unwrap();

    let document = Html::parse_document(&html);
    let selector = Selector::parse("a").unwrap();

    for el in document.select(&selector) {
        if let Some(href) = el.value().attr("href") {
            let text = el.text().collect::<Vec<_>>().join(" ");
            println!("{} → {}", text.trim(), href);
        }
    }
}
```

---

### 7.13 Basic Web Crawler — BFS

```rust
use std::collections::{HashSet, VecDeque};
use scraper::{Html, Selector};

async fn fetch(url: &str) -> Option<String> {
    reqwest::get(url).await.ok()?.text().await.ok()
}

fn extract_hrefs(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    document
        .select(&selector)
        .filter_map(|el| el.value().attr("href").map(String::from))
        .collect()
}

async fn crawl(seed: &str) {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    queue.push_back(seed.to_string());

    while let Some(url) = queue.pop_front() {
        if visited.contains(&url) {
            continue; // URL Seen? check
        }
        visited.insert(url.clone());

        if let Some(html) = fetch(&url).await {
            let links = extract_hrefs(&html);
            for href in links {
                if !visited.contains(&href) {
                    queue.push_back(href); // feed URL Frontier
                }
            }
        }
    }
}
```

---

### 7.14 Concurrent Fetching — SDE2 Follow-up

**Problem:** sequential fetching is slow — one thread waiting on HTTP at a time.

**Solution:** spawn async tasks, await all in parallel.

```rust
use futures::future::join_all;

async fn fetch_all(urls: Vec<String>) -> Vec<Option<String>> {
    let handles: Vec<_> = urls
        .into_iter()
        .map(|url| tokio::spawn(async move {
            reqwest::get(&url).await.ok()?.text().await.ok()
        }))
        .collect();

    join_all(handles)
        .await
        .into_iter()
        .map(|r| r.ok().flatten())
        .collect()
}
```

> **SDE2 follow-up:** *"How do you avoid overwhelming a server?"* — Use a semaphore to cap concurrency.

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

let sem = Arc::new(Semaphore::new(10)); // max 10 concurrent requests

let permit = sem.clone().acquire_owned().await.unwrap();
tokio::spawn(async move {
    let _permit = permit; // dropped when task ends → releases slot
    fetch(&url).await
});
```

---

### 7.15 Dynamic JavaScript Pages

> **Key distinction:** Static HTML scraping ≠ browser-rendered DOM. `reqwest` fetches what the server sends over the wire — not what the browser builds after running JavaScript. These are two completely different documents for JS-heavy sites.

**Problem:** `reqwest` + `scraper` only see the raw HTML the server sends. JavaScript-rendered content is invisible.

```text
Browser renders:           reqwest sees:
<div id="price">99</div>   <div id="price"></div>
```

**Solution — reach for a headless browser:**

| Tool | Language | Notes |
|---|---|---|
| `fantoccini` | Rust | WebDriver client |
| Playwright | Node.js | Most mature for JS-heavy sites |
| Headless Chromium | Any | Via ChromeDriver or CDP |

---

### 7.16 robots.txt — The Contract You Must Honor

Every crawler must check `robots.txt` before crawling a site.

```text
https://www.example.com/robots.txt
```

Example:

```text
User-agent: Googlebot
Disallow: /private/
Disallow: /admin/
```

- Cache the file — don't re-fetch it on every request.
- Refresh the cache periodically (e.g., every 24h via cron).
- Violating `robots.txt` is not illegal but is hostile and gets you IP-banned.

---

### 7.17 Extending the Crawler

The system is designed to be modular. New content types plug in after Content Parser:

```text
Content Parser output
    ├── Link Extractor   ← default module, always present
    ├── PNG Downloader   ← plug in when you need images
    └── Web Monitor      ← plug in for copyright detection
```

Other extension points:
- **Server-side rendering module** — run JS before parsing, for dynamic pages.
- **Anti-spam filter** — drop low-quality or spam pages before storing.
- **Analytics pipeline** — track crawl health, page error rates, freshness metrics.

---

## Common Rust Patterns Used in Crawlers

These patterns come up repeatedly — both in implementation and in discussions.

| Pattern | Purpose |
|---|---|
| `Result<T, E>` | Handle fallible operations (HTTP errors, parse failures) |
| `?` operator | Propagate errors ergonomically without panicking |
| `Option` handling | Deal with missing attributes (`href`, `text`) |
| Iterators + `filter_map` | Chain transformations on element collections cleanly |
| `HashMap` / `HashSet` | Visited URL tracking, deduplication |
| `async/await` + `tokio` | Concurrent page fetching without blocking threads |
| `Arc<Mutex<T>>` | Share crawler state across async tasks |

> **`Arc<Mutex<T>>` in a crawler:** The visited `HashSet` and URL queue must be shared across worker tasks. `Arc` gives shared ownership; `Mutex` gives exclusive write access.

> **`tokio::sync::Mutex` vs `std::sync::Mutex`:** In production async Rust, prefer `tokio::sync::Mutex` inside async tasks. `std::sync::Mutex` blocks the thread while locked — in an async context this can stall the entire tokio worker thread. `tokio::sync::Mutex` yields instead, keeping the async runtime responsive.

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

let visited = Arc::new(Mutex::new(HashSet::<String>::new()));

let visited_clone = visited.clone();
tokio::spawn(async move {
    let mut set = visited_clone.lock().unwrap();
    set.insert("https://example.com".to_string());
});
```

---

## Quick Reference

| Concept | Rule / Formula |
|---|---|
| Scraper vs Crawler | Scraper extracts, Crawler discovers |
| Traversal strategy | BFS-like traversal preferred for large-scale crawlers (DFS valid for focused/depth-sensitive crawlers) |
| Politeness | One host → one queue → one worker → delay between requests |
| Deduplication (content) | Hash page body; discard on match |
| Deduplication (URL) | Bloom filter — no false negatives |
| DNS bottleneck | Cache DNS responses (10–200ms per lookup) |
| Scale: QPS | 1B pages/month ≈ 400 QPS, peak 800 |
| Scale: storage | 500 TB/month, 30 PB for 5 years |
| Spider trap defense | Cap max URL length; flag anomalous URL volume per domain |
| JS-rendered pages | Use headless browser (fantoccini / Playwright) |
| Concurrency limit | Semaphore with bounded permit count |
