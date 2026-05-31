# Ephemerust integration plan (Rusty_Server)

This document defines an **iterative, approval-gated** path to:

1. Rename project references from **CLI_Astro_Calc** to **Ephemerust** across Rusty_Server docs (including `OVERVIEW.md`, `DEVELOPMENT_PLAN.md`, and `ReadMe.md`).
2. Add **Ephemerust** as a **Cargo dependency** (path, git, or **crates.io** once published), expose **`/api/v1/ephemeris/...`** JSON APIs backed by the `ephemerust` crate, and align **MSRV** with Ephemerust’s `rust-version`.

**Governance**

- Work proceeds in **phases**. Each phase ends with an **approval gate**: no implementation in the next phase until you explicitly approve (reply in issue/chat or tick the gate checklist in a PR).
- Prefer **small PRs** (one phase or sub-phase per PR) so review and rollback stay easy.
- **Non-goals for initial phases**: hosting a separate Ephemerust HTTP binary, re-implementing NOAA/DONKI inside Ephemerust, or CelesTrak polling in Rusty_Server (Ephemerust `network` / `--tle-url` can remain a later joint milestone).

---

## Phase 0 — Preconditions and decisions (informational; no code)

**Status:** **Complete** — path dependency + API prefix locked for this showcase/educational repo (no expectation of third-party clones without the sibling Ephemerust tree).

**Objectives**

- Confirm how Rusty_Server will resolve **Ephemerust**: local path, git pin, or **crates.io** (after you publish).
- If not using crates.io: confirm machine layout for **path** (sibling **`Ephemerust`** checkout next to this repo) vs **git** for CI and collaborators without that folder.
- Confirm Ephemerust **SemVer / API churn** tolerance: `ephemerust` is `0.x` and may break between minors (true whether from git or crates.io).

**Decision record (fill before Phase 2)**

| Topic | Options | Notes |
|-------|---------|--------|
| Dependency source | **A.** `path = "../Ephemerust"` (sibling checkout) | Best for tight co-development; requires folder layout. |
| | **B.** `git = "…"` + `rev` or `tag` | Reproducible; no local sibling; good for CI. |
| | **C.** `ephemerust = "0.x.y"` on **crates.io** | Same ergonomics as other deps; needs **crate name available** on crates.io and a **published version**; pin minor/patch (or exact) for reproducibility. Does **not** replace Phase 0—you still pick A, B, or C (or document “crates.io default + path override for local hacking”). |
| Public URL prefix | `/api/v1/ephemeris/...` | Alternative: `/api/v1/astro/...`. Changing only affects docs and clients. |

Publishing Ephemerust to **crates.io** does **not** change Phase **1–5** (docs, MSRV, API design, implementation slices). It only **extends Phase 0**: you may choose **Option C** as the canonical dependency for Rusty_Server and keep **path** as an optional `[patch]` or workspace note for active Ephemerust development.

**Recorded decision (repository owner — local co-development)**

- **Dependency source:** **Option A — `path`** to the sibling Ephemerust checkout (`../Ephemerust` relative to this repo), since both projects live alongside each other for development and showcase. **No expectation** that others clone Rusty_Server alone; if that ever changes, switch to git or crates.io and document layout.
- **Public URL prefix:** **`/api/v1/ephemeris/...`** for future HTTP handlers (Phase 3–4).

**Gate 0 — Approval**

- [x] Default dependency strategy: **path** (sibling checkout) — *recorded above*; **`Cargo.toml` includes `ephemerust`** (see Phase 2.1).
- [x] API prefix: **`/api/v1/ephemeris`** — *confirmed for integration plan and future routes*.

---

## Phase 1 — Documentation rename (CLI_Astro_Calc → Ephemerust)

**Status:** **Complete** (2026-05-31) — product/docs names use **Ephemerust**; **`../Ephemerust`** denotes the **on-disk sibling folder** for the Cargo `path` dependency. **Owner approval** to treat Phase 1 as final was given when proceeding to Phase 2.

**Objectives**

- Align naming with the upgraded library/repo: **Ephemerust** ([`IsomorphicAlgo/Ephemerust`](https://github.com/IsomorphicAlgo/Ephemerust)).
- Remove stale “CLI_Astro_Calc” references where they mean “the calculation engine we integrate,” without rewriting history or unrelated personal notes unless you want full consistency.

**Scope (files to update in Rusty_Server)**

| File | Action |
|------|--------|
| `ReadMe.md` | Replace CLI_Astro_Calc with Ephemerust; link GitHub; clarify “Ephemerust (formerly CLI_Astro_Calc)” once if helpful for readers. |
| `OVERVIEW.md` | Same; update architecture diagram labels if they say `CLI_Astro_Calc`. |
| `DEVELOPMENT_PLAN.md` | Rename Priority E / Phase references; point **Priority E** to this file (`EPHEMERUST_INTEGRATION_PLAN.md`) and/or keep a short summary with link to full plan. |
| `EPHEMERUST_INTEGRATION_PLAN.md` | (this file) — add “Last updated” when phases complete. |
| Other repo text | Grep `CLI_Astro_Calc`, `CLI_Astro`, `CLI_INTEGRATION` and fix **in-repo** references (`prompt.md`, `FOLDER_ORGANIZATION_SUMMARY.md`, `SECURITY.md` if any, static HTML copy if any). |
| Broken links | Replace `Guides/CLI_INTEGRATION_PLAN.md` with either this plan or a thin stub `Guides/CLI_INTEGRATION_PLAN.md` that redirects to Ephemerust + this plan (optional). |

**Out of scope**

- If you **move** the Ephemerust checkout again, update `path` in `Cargo.toml` and the paths documented in `ReadMe.md` / this plan.

**Deliverables**

- Single PR: “docs: Ephemerust naming and integration pointers.”
- Grep-clean for intended strings (allowlist `CHANGELOG` or git history only if needed).

**Gate 1 — Approval**

- [x] Doc rename approved and applied (`DEVELOPMENT_PLAN.md`, `ReadMe.md`, `OVERVIEW.md`, `FOLDER_ORGANIZATION_SUMMARY.md`, `prompt.md`, this plan).
- [x] Proceed to Phase 2 (Cargo dependency already satisfied for **path**; next optional work: Phase 2.3 MSRV, then Phase 3+).

---

## Phase 2 — Cargo dependency (2.1)

**Status:** **Complete** — `ephemerust` via `path = "../Ephemerust"`; **`cargo check`** / **`cargo test --lib`** verified. **Owner approval:** Phase **2** (including **2.3** MSRV) explicitly **approved** when requesting **Phase 3** kickoff.

**Objectives**

- Add `ephemerust` to `Rusty_Server` so library code can call `ephemerust::…` without shelling out to the CLI.

**Note:** Closing **Phase 0** already added the **`path`** dependency line to `Cargo.toml`. The steps below remain the reference if you switch to git or crates.io later; otherwise Phase 2.1 for **path** is already satisfied.

**Implementation steps**

1. In root `Cargo.toml`, under `[dependencies]`:

   **Path (local sibling)**

   ```toml
   ephemerust = { path = "../Ephemerust" }
   ```

   Adjust `../Ephemerust` so it resolves from the **Rusty_Server repo root** (same parent directory as today’s layout).

   **Git (pinned)**

   ```toml
   ephemerust = { git = "https://github.com/IsomorphicAlgo/Ephemerust", rev = "<full-commit-sha>" }
   ```

   Prefer **`rev`** over a floating `branch` for reproducible builds until `ephemerust` reaches a semver you trust.

   **Crates.io (after publish)**

   ```toml
   ephemerust = "0.2.0"   # example: pin to the version you validated
   ```

   Treat version pins like git `rev`: bump intentionally when Ephemerust releases breaking `0.x` changes.

2. `cargo check` on Rusty_Server; resolve any **feature** needs (Ephemerust default features are empty; avoid enabling `network` on the server unless you explicitly need stub/TLE-URL behavior in-process).

3. Document the choice in `ReadMe.md` (Development / Ephemerust subsection): e.g. crates.io for everyone, or path for local co-dev + crates.io in default manifest.

**Gate 2 — Approval**

- [x] `cargo check` passes with chosen dependency form (**path** `../Ephemerust`).
- [x] Locking strategy: **path** to sibling checkout (no git `rev` / crates.io pin required for current workflow).
- [x] Phase 2.3 (MSRV) completed in same pass — see below.

---

## Phase 2.3 — MSRV alignment

**Status:** **Complete** — `rust-version = "1.88"` in `Cargo.toml`; repo-root **`rust-toolchain.toml`** pins **1.88**; `ReadMe.md` and `Troubleshooting/BUILD_TROUBLESHOOTING.md` updated.

**Problem**

- `ephemerust` declares `rust-version = "1.88"` in its `Cargo.toml`.
- `rusty-server` does not declare `rust-version` today; CI and contributors may use older toolchains and fail with opaque errors.

**Objectives**

- Make MSRV **explicit and consistent** for Rusty_Server when Ephemerust is a dependency.

**Implementation steps**

1. Add to **`Rusty_Server/Cargo.toml`** (root package):

   ```toml
   rust-version = "1.88"
   ```

   (Match Ephemerust unless you negotiate a lower MSRV upstream—**do not** set lower without verifying Ephemerust builds.)

2. Update contributor docs (`ReadMe.md`, `Troubleshooting/BUILD_TROUBLESHOOTING.md` if applicable): “Rust **1.88+** required.”

3. If you use **GitHub Actions** / other CI: set `rust-toolchain.toml` **or** CI `dtolnay/rust-toolchain` to **1.88** (or `stable` once stable ≥ 1.88 on your runners).

4. Optional: add `rust-toolchain.toml` at repo root:

   ```toml
   [toolchain]
   channel = "1.88"
   ```

   This maximizes reproducibility; some teams prefer CI-only pinning—choose at Gate 2.3.

**Gate 2.3 — Approval**

- [x] Toolchain **1.88** via `rust-toolchain.toml` (local `rustup` uses it in this repo).
- [x] `cargo check` / `cargo test --lib` run on pinned toolchain (verify after edits).
- [x] **`rust-toolchain.toml`** chosen (no GitHub Actions in repo; CI-only N/A).
- [ ] Proceed to **Phase 3** (API design) when ready.

**Ordering note**

- **Done:** Phases **2.1** (path dep) and **2.3** (MSRV + `rust-toolchain.toml`) were applied together per plan.

---

## Phase 3 — API design (`/api/v1/ephemeris/...`) (2.2)

**Status:** **Complete (v0.1)** — full contract in [`Guides/API_EPHEMERIS.md`](Guides/API_EPHEMERIS.md). **Phase 4** handlers and integration tests are implemented (2026-05-31).

**Objectives**

- Define JSON contracts and routes **before** large handler code.
- Map Ephemerust **library** capabilities to a minimal first slice (MVP), then extend.

**Suggested route layout (MVP → extensions)**

| Method | Path | Purpose (MVP) |
|--------|------|----------------|
| `POST` | `/api/v1/ephemeris/time` | JD / GMST for a UTC instant (wraps `ephemerust::time` entrypoints). |
| `POST` | `/api/v1/ephemeris/position` | Planet / Sun / Moon RA/Dec for a date (wraps `celestial` / `planets` as appropriate). |
| `POST` | `/api/v1/ephemeris/satellite/track` | TLE + instant + observer → state / subpoint / look (subset of CLI `track` modes). |

**Why POST**

- Bodies can carry **TLE text** and structured observer fields without URL length limits.
- Aligns with “deserialize JSON → compute → JSON” requirement.

**Cross-cutting**

- **Auth / rate limits**: Register new routes on the same `Router` as existing `/api/v1/...` routes in `src/api/routes.rs` so they inherit existing middleware unless you exempt ephemeris (default: **same** policy).
- **Errors**: Map `ephemerust::Result` / `AstroError` to HTTP **400** (bad input) vs **422** (semantic validation) vs **500** (unexpected); documented in [`Guides/API_EPHEMERIS.md`](Guides/API_EPHEMERIS.md).
- **Timeouts**: Long pass prediction — caps in **API_EPHEMERIS.md** (`predict_passes_hours` max 168, `ground_track_hours` max 24 for MVP).

**Deliverables**

- [x] [`Guides/API_EPHEMERIS.md`](Guides/API_EPHEMERIS.md) — request/response schemas, limits, error model, implementation order for Phase 4 slices.
- [ ] Optional: PR with route stubs returning **501** (deferred unless you want URLs live before handlers).

**Gate 3 — Approval**

- [x] MVP route list and POST+json pattern — **as in `Guides/API_EPHEMERIS.md` v0.1** (owner: begin Phase 3).
- [x] Error and size limits policy — **as in `Guides/API_EPHEMERIS.md` v0.1**.
- [x] **Proceed to Phase 4** (implement handlers + tests) — *implemented 2026-05-31*.

---

## Phase 4 — Implementation (2.2) — sliced

**Status:** **Complete** (2026-05-31) — `src/api/ephemeris_handlers.rs`, routes under `/api/v1/ephemeris/*`, integration tests `tests/ephemeris_api_test.rs` (lazy DB pool; no MySQL required for these tests). `cargo test --lib` and `cargo test --test ephemeris_api_test` verified.

**Objectives**

- Implement handlers: **JSON → ephemerust → JSON** under `src/api/` (new module e.g. `ephemeris_handlers.rs` or `handlers/ephemeris.rs`).
- Wire routes in `src/api/routes.rs` (mirror existing patterns).

**Iteration slices (each slice = optional separate PR + mini-gate)**

1. **Slice 4a — `time`**: smallest integration proof; establishes serde models, error mapping, tests.
2. **Slice 4b — `position`**: slightly more input validation.
3. **Slice 4c — `satellite/track`**: strictest validation (TLE length, observer lat/lon); optional feature flag `ephemeris-api` if you ever need to build server without Ephemerust (usually unnecessary).

**Tests**

- **Unit**: serde round-trip, invalid body cases.
- **Integration**: `tower::ServiceExt` / axum tests hitting new endpoints with golden JSON (no network).

**Gate 4 — Approval (per slice or once at end)**

- [x] Tests green (`cargo test --lib`, `cargo test --test ephemeris_api_test`).
- [x] Satellite slice merged with the `time` and `position` handlers (single PR surface).
- [x] Proceed to Phase 5 (optional doc polish) — *completed 2026-05-31*.

---

## Phase 5 — Docs, discovery, and Ephemerust roadmap hygiene (optional but recommended)

**Status:** **Complete** (2026-05-31) — `ReadMe.md` (Testing the API + ephemeris `Invoke-WebRequest` / `curl`), `DEVELOPMENT_PLAN.md` (Phase 11 Ephemerust note), `OVERVIEW.md`, `Guides/API_DOCUMENTATION.md` + `Guides/API_EPHEMERIS.md` status, sibling **`Ephemerust/docs/roadmap.md`** Phase 2 re-scope.
**Rusty_Server**

- Update `ReadMe.md` “Testing the API” with one `ephemeris` example (`Invoke-WebRequest` or `curl`).
- Update `DEVELOPMENT_PLAN.md` **Phase 11** note: “TLE/SGP4 geometry may leverage Ephemerust library already used by `/api/v1/ephemeris`” to avoid duplicate engines.

**Ephemerust repo (separate PR there)**

- Adjust `docs/roadmap.md` **Phase 2** so it does not duplicate Rusty_Server’s NOAA stack; instead reference Rusty_Server as the deployment host or scope Phase 2 to “optional thin HTTP for standalone demos.”

**Gate 5 — Approval**

- [x] Rusty_Server doc edits applied (this PR / session).
- [x] Ephemerust `docs/roadmap.md` Phase 2 aligned with Rusty_Server as operations host.
- [ ] Optional: your explicit sign-off on cross-repo wording (no code dependency).
---

## Appendix A — Approval checklist (copy into PR description)

```
Phase: [0 | 1 | 2 | 2.3 | 3-design | 4a | 4b | 4c | 5]
Gate preconditions met: [link to filled Decision record]
MSRV: 1.88 verified (cargo + rustc)
Tests: cargo test
Docs: ReadMe / OVERVIEW / DEVELOPMENT_PLAN / API ephemeris section updated: [y/n]
```

---

## Appendix B — Post-integration backlog (not gated here)

- Ephemerust **`network`** / CelesTrak: either implement in Ephemerust per `http_plan.md`, then optionally expose “fetch TLE by URL” through Rusty_Server with **strict** rate limits and caching—or keep TLE fetch in Rusty_Server with `reqwest` and pass text into Ephemerust.
- **Phase 11** persistence: store TLE history in MySQL; use Ephemerust for propagation only.
- Dashboard: small UI panel calling `/api/v1/ephemeris/...` (separate UX gate).

---

**Document owner**: Rusty_Server maintainers.  
**First gate**: Complete **Gate 0** before merging dependency or API code.

### Phase 1 follow-up

Phase 1 (documentation rename) is **complete**. The **`ephemerust`** crate is resolved via **`path = "../Ephemerust"`** (sibling directory alongside `Rusty_Server`, e.g. `C:\Users\micha\Rust\Ephemerust`). The string *CLI_Astro_Calc* may still appear in docs only as **historical** context for the old project name, not as a directory name.
