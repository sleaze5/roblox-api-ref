# roblox-api-ref

Unofficial Roblox Engine API reference. Search and browse every class, member, enum, and data type, and see what changed in each Roblox Studio build.

The site is static. The build generates it from `https://setup.rbxcdn.com/${version}-Full-API-Dump.json`, from `en-us.json` inside `https://setup.rbxcdn.com/${version}-content-api-docs.zip`, and from the Studio class icons inside `https://setup.rbxcdn.com/${version}-content-studio_svg_textures.zip`.

## Technology stack

- Build: Rust (pinned in `rust-toolchain.toml`), a Cargo workspace in `crates/`. Maud renders the pages, Serde reads and writes the data, reqwest and Tokio download the API files, Rayon renders pages in parallel, and minify-html minifies them.
- Search: a Rust engine compiled to WebAssembly with wasm-bindgen, loaded on first use, over a compact binary index (`search.bin`).
- Client: vanilla TypeScript `7` in `src/client`, with no framework.
- Tooling: Vite+ (Vite, Rolldown, Oxlint, Vitest), Bun (runtime, package manager, `bun test`, and `bun fmt`, its built-in Oxfmt), Cargo (rustfmt, Clippy), Playwright for browser tests
- Design: Tailwind CSS `4`, Lightning CSS, Geist, Geist Mono, Phosphor icons

## Layout

- `crates/routes`: the URL of every page, shared by the renderer and the search engine.
- `crates/core`: the API model, the dump and diff formats, and generation: normalizing the API dump, applying the API docs, building the update history, the statistics, the class icon sprite, and the search index. `core::js` reproduces the JavaScript behavior that the output depends on, such as `Intl.Collator` name order, number formatting, and date formatting.
- `crates/search`: the search engine and the binary index format. `crates/search-wasm` exposes it to the browser as `Search` and `Matcher`.
- `crates/site`: the Maud templates for every page, their Markdown versions, head tags, the feed, the sitemap, and the build that writes `dist/`.
- `crates/dump`: downloading the API files and computing diffs.
- `crates/cli`: the `rapi` binary. `cargo rapi help` lists its commands.
- `src/client`: the scripts that run in the browser. `src/edge`: the Markdown negotiation for Cloudflare and `bun vp run preview`. `src/styles`: the CSS. `vite/`: the Vite plugin that runs `rapi`, and the preview server. `scripts/`: font subsetting and dependency updates. `tests/browser`: Playwright tests of the built site.

## Task orchestration

Interact with the project only through `bun vp run <task>`. Do not run Vite, Oxlint, Vitest, or `bun fmt` directly, and do not install Node.js or a global `vp`. Bun and Rust (through `rustup`) are the only tools to install. `bun vp run setup` installs the packages and generates the site data. `bun install` installs `vp` from the `vite-plus` package. Run `bun vp run` to list the available tasks.

Tasks are defined in `run.tasks` in `vite.config.ts`. Vite+ runs the tasks in `dependsOn` first and caches task results.

`bunfig.toml` sets `run.bun`, so `bun vp` runs `vp`, and every tool that it starts, on Bun instead of Node.js. In a task command, start a tool with `bun vp <command>`, and run another task with a bare `vp run <task>`. Vite+ runs a bare `vp run` in the same task graph. `bun vp run` inside a task starts a nested runner, which fails. Run Rust through `cargo rapi <command>`, the alias in `.cargo/config.toml` for the release build of the `rapi` binary.

- `dump:update` runs `cargo rapi dump-update`. It reads the LIVE `WindowsStudio64` version from `clientsettings.roblox.com`. Only Studio uploads include the API dump and the API docs. When that build is newer than the last version in `roblox-versions.json`, the task downloads both API dumps and both English API docs into `data/`, writes the changes between them to `diffs/<version>.json`, and appends the build and its publish date to `roblox-versions.json`. When `roblox-versions.json` is empty, the task starts from the Studio build before the live one in [version-history.json](https://github.com/MaximumADHD/Roblox-Client-Tracker/blob/roblox/version-history.json). It skips a new version until its API dump and API docs are on the CDN. `cargo rapi diff <from> <to>` rewrites the diff between two tracked versions. `cargo rapi backfill` prepends every build in [RobloxAPI/build-archive](https://github.com/RobloxAPI/build-archive) older than the first version in `roblox-versions.json`, one per version, and writes their diffs from the standard `API-Dump.json`, because builds before 0.553 have no full dump. The diff into the first tracked version also compares standard dumps. Backfilled diffs have no `descriptions`, and the diff into the first tracked version lists the APIs that only the full dump has in `unrecorded`.
- `deps:update` updates each package in `package.json`, and the Bun version in `devEngines`, to the newest release that keeps the same major version. It skips deprecated releases. A prerelease version moves along its own line, such as `2.0.0-rc.15` to `2.0.0-rc.16`, or to the stable release of the same version, such as `2.0.0`. It also moves `rust-toolchain.toml` to the newest stable Rust, and `rust-version` in `Cargo.toml` with it. Then it runs `bun install` to update `bun.lock`.
- Dependabot updates the GitHub Actions versions, the packages in `package.json`, and the crates in `Cargo.toml` every week with the same rules, except the Bun version, the Rust toolchain, and the packages that the `bun` entry in `.github/dependabot.yml` ignores completely. Dependabot cannot update npm aliases such as `vite`, and `vite-plus` must move with that alias, so it ignores both. `bun scripts/deps/update.ts --uncovered` updates only the Bun version, the Rust toolchain, and the packages that Dependabot ignores. The `Update dependencies` workflow runs it every Monday, or updates everything, including `cargo update`, when you start it by hand, then installs the new Bun version, runs `bun install`, and lands the commit through the `deps/update` branch with the `Land` action. `wasm-bindgen` and `wasm-bindgen-cli-support` are pinned to the same exact version, because the generated glue must match the library.
- `dump` runs `cargo rapi dump`. It downloads the API dump, the English API docs, and the Studio class icons for the last version in `roblox-versions.json` into `data/`, unless they are already downloaded. It reads `en-us.json` out of `content-api-docs.zip` with HTTP range requests instead of downloading the whole archive. It reads the dark `Shared\InsertableObjects\Dark\Standard\<Class>@3x.png` icons out of `content-studio_svg_textures.zip` the same way, fetching nearby entries with one range request, and saves them as base64 in `Studio-Icons-<hash>-<channel>.json`. It deletes every other dump, API docs, and icons file in `data/`.
- `gen` runs `dump`, then `cargo rapi gen` and `bun scripts/fonts.ts`. They write what the client and the edge import into `src/generated/`: the search index `search.bin`, the search engine `search_bg.wasm` with its glue `search.js`, built through Cargo for `wasm32-unknown-unknown` with the `wasm` profile and wasm-bindgen as a library, the icon sprite, the favicon, the icon masks, the class icon sprite `class-icons.png` with its grid size in `class-icons.css`, the subset fonts, `build.json`, and `pages.json`, which lists every page with its Markdown path for the edge tests. It is never cached.
- `dev`, `web:test`, `test:watch`, `typecheck`, `web:check`, and `bench` run `gen` first, and `rust:test` runs `dump` first. `dev` starts the Vite dev server. The `pages` plugin in `vite/pages.ts` renders each page on request through one long-lived `cargo rapi dev` process, which loads the site data once. A change to `crates/`, `data/`, `diffs/`, or `roblox-versions.json` restarts that process and reloads the page.
- `build` runs `fix`, then builds the client with Vite, then runs `cargo rapi render`, which generates the site data and writes every page, its Markdown version, the feed, the sitemap, `robots.txt`, and `opensearch.xml` into `dist/`. It fills the `index.html` that Vite built, and reads hashed asset URLs from the Vite manifest. `preview` runs `build` first.
- `dist` runs `gen`, then the same Vite build and render as `build`, without `fix`. It never changes source files. Cloudflare deploys with it.
- `format:check` and `format:fix` run `cargo fmt` and `bun fmt` with the `fmt` options in `vite.config.ts`. `clippy` runs Clippy with every warning as an error. `web:check` runs `vp check --no-fmt`, which lints and typechecks. `check` runs `format:check`, `clippy`, then `web:check`.
- `rust:test` runs the Rust tests in the release profile, and `web:test` runs Vitest. `test` runs both. The Rust integration tests in `crates/core/tests` and `crates/site/tests` generate and render the site from `data/` and `diffs/`.
- `test:browser` runs the Playwright tests in `tests/browser` against `dist/` through the preview server. Build first. It also checks that every class in the built pages and the client scripts has CSS rules, which replaces a Tailwind class lint.
- `bench` runs `scripts/bench.ts`, which measures generation, the client build, prerendering, search and filter latency in WebAssembly, payload sizes, and peak memory. It builds the client into `node_modules/.cache/bench-dist` with `RAPI_RENDER=0`, which makes the `pages` plugin skip prerendering, then times `cargo rapi render` on its own.
- When `RAPI_BIN` names a built `rapi` binary, the `pages` plugin renders with it instead of `cargo rapi`. CI sets it, so the jobs that build the site need no Rust toolchain. Run a task with `--ignore-depends-on` to skip `gen` when `src/generated/` and `data/` are already current.
- `fix` runs `lint:fix`, `format:fix`, and `check`, in that order.
- `ci` runs `check`, `test`, and a build. It never changes files.

Do not add or remove tasks unless the request requires it. When you add a task, set `cache: false` if the task uses the network, writes source files, or runs a server. Vite+ cannot see the files that a Bun or Cargo process reads, so give every other new task that runs Bun `sourceInput` in `cache.input`.

## Versions and diffs

- `roblox-versions.json` lists every version that the site has published, oldest first. The last entry is the current version. Each entry names a Studio build by its `version` and `hash`.
- `diffs/<version>.json` holds the changes from the previous entry in `roblox-versions.json` to `<version>`. Commit the diffs and `roblox-versions.json`. Do not commit API dumps or Studio icons.
- `dump:update` writes both files. Do not edit them by hand.
- A diff lists changes in the shape of the API dump: `element` is `Class`, `Member`, `Enum`, or `EnumItem`, and `type` is `Add`, `Remove`, or `Change`. `value` holds the dump entry after the change, or before it for `Remove`. A `Change` also lists each changed dump field with its `before` and `after` values. An added class or enum also gets an `Add` for each of its members or items. A removed class or enum does not list its members or items.
- A diff also lists `descriptions`: each description in `en-us.json` that was added, changed, or removed for a class, member, parameter, return value, enum, or enum item that exists in the new version. `part` is `description`, `parameter`, or `return`, and `before` and `after` hold the HTML, or `null` when there is none. A member or enum item change also holds its dump entry in `value`. Roblox publishes new API docs once per release, so builds of the same release usually have no description changes. A diff without `descriptions` did not record them, and the site shows its description changes as not recorded and leaves it out of the description statistics.
- A diff may list `unrecorded`: `Add` entries for APIs that exist from that version on but that no earlier diff could see, because older versions only have the standard `API-Dump.json`. The site shows their addition as unrecorded.
- An update with no changes is a rebuild.

## Hosting and agents

- The site is a static build hosted on Cloudflare Pages at `https://rapi.sleaze.dev/`. Cloudflare's Git integration builds `dist/` and deploys every push to the default branch as production, and every push to another branch as a preview. No GitHub workflow deploys.
- Cloudflare runs `bash scripts/cloudflare-build.sh` with `SKIP_DEPENDENCY_INSTALL=1` and no `BUN_VERSION`, and serves `dist/`. The script installs the exact Bun version in `devEngines` in `package.json`, installs `rustup` when it is missing and the toolchain in `rust-toolchain.toml`, and runs `bun install --frozen-lockfile` and `bun vp run dist`. It does not check or test, because GitHub CI has already passed on every commit that reaches the default branch. The build does not precompress files, because Cloudflare compresses responses itself. `package.json` is the only place that names the Bun version.
- `site_origin` in `crates/site/src/context.rs` resolves the origin at build time from `SITE_ORIGIN`, then `CF_PAGES_URL`, then `http://localhost:5173`. Production sets `SITE_ORIGIN` in Cloudflare. Previews use their own `CF_PAGES_URL`. The `build`, `rust:test`, `web:test`, and `ci` tasks list both variables in `cache.env`, because `vp run` passes only listed variables to a cached task. `dist` is not cached, so it gets every variable. Every absolute URL in pages, Markdown, the sitemap, and JSON-LD uses this origin. Do not hardcode the production domain. Tests render with their own origin, not a literal production origin.
- Every page has a Markdown version. Its URL is the page URL without the trailing slash, followed by `.md`. The Markdown version of the home page is `llms.txt`, and the Markdown version of the not found page is `404.md`.
- `functions/_middleware.ts` is a Cloudflare Pages Function that negotiates Markdown with `src/edge/negotiate.ts`. A page request that prefers `text/markdown` in its `Accept` header gets the Markdown version from the same URL, or `404.md` with status 404 when the page does not exist. Other page requests get the HTML with a `Link` header that points to the Markdown version. Every negotiated response has `Vary: Accept`. A request that accepts neither HTML nor Markdown gets 406. `bun vp run preview` serves `dist/` through the same negotiation.
- `public/_routes.json` keeps the Function off `/assets/`, every Markdown file, and the files at the site root, so that those requests stay free static requests. Every page route must run the Function.
- `public/_headers` caches the files in `/assets/` for a year as immutable, because their names change with their content. Cloudflare strips the `ETag` from HTML, so the Function stamps every page and its Markdown with `Last-Modified` set to the build time from `src/generated/build.json`, and answers 304 to `If-Modified-Since` from the same build.
- Write absolute links in Markdown, so that the same Markdown works from any URL.
- The build writes an Atom feed of the 20 newest updates to `/updates.xml`. Every page except the not found page links to it. `public/_routes.json` keeps the Function off it, and `public/_headers` serves it as `application/atom+xml`.
- GitHub holds the source and runs the automation. The workflows are built from composite actions in `.github/actions`:
    - `setup-bun` installs the Bun version in `devEngines` with `setup-bun` and runs `bun install --frozen-lockfile` with the Bun install cache. `setup-rust` installs the toolchain in `rust-toolchain.toml` and restores the compiled dependencies for one kind of build, `check` for Clippy or `release` for `rapi` and the tests. Checkout gives every source file a new time, so Cargo rebuilds the workspace crates in every job, and the cache keeps only the dependencies. `data` restores `data/` for the last version in `roblox-versions.json`.
    - `generate` caches `src/generated/`, `data/`, and the `rapi` binary under a hash of every file that `gen` reads. On a hit it only looks the key up, and otherwise it builds `rapi` and runs `gen`. `generated` restores that cache in a later job and sets `RAPI_BIN`.
    - `changes` reads the files that changed since the merge base with the base branch and selects the parts to check: `rust` for the Cargo workspace, `data` for `roblox-versions.json` and `diffs/`, and `web` for the client, the edge, the tests, the scripts, and the packages. Markdown selects nothing. Any other path, such as a workflow, and an empty diff select every part.
    - `playwright` installs Chromium for the Playwright version in `package.json`, with the browser cache.
- The CI workflow runs on every push to a branch other than the default branch, every fork pull request, and by hand, where it checks everything or only what changed. Its jobs run in parallel: `format` on every change, `clippy` when Rust changed, `rust-test` when Rust or the data changed, `generate` for any other change, and then `web-check` when Rust or the client changed, and `web-test` and `site`, which builds `dist/` and runs the browser tests, for any other change. `build` passes when every job that ran passed, and it is the only check that the default branch requires. The default branch accepts a commit only after `build` passes on it, so CI does not run again on pushes to the default branch. A repository admin can bypass that rule only by merging a pull request.
- A run restores only the caches of its own branch and of the default branch, so only the default branch saves the Cargo caches. The `Warm caches` workflow saves them there, with the `generate` cache and the compiled Rust tests, on each push to the default branch, after each landed commit, every week, and by hand.
- The `Land` action pushes a workflow's commit to a branch, starts CI on that branch for what the commit changes, because pushes made with `GITHUB_TOKEN` do not start workflows, pushes the same commit to the default branch once CI passes, and starts `Warm caches`. Workflows do not open pull requests, because GitHub holds the workflow runs of a pull request that `GITHUB_TOKEN` opens until a person approves them. The hourly release workflow first compares the LIVE Studio version with the last entry in `roblox-versions.json` and stops before any setup when they match. A run started by hand always looks for a new version. It lands each new Roblox version through the `release/update` branch. Dependabot pull requests merge themselves once CI passes. Every change to the default branch starts a Cloudflare deploy.

## Design

Read [docs/DESIGN.md](./docs/DESIGN.md) before you change UI, styles, icons, interface text, or motion. It defines the tokens, components, layout, and motion of the site.

## Animation performance

- Animate only `transform` and `opacity`. Hover and focus feedback may also transition `color`, `background-color`, and `border-color`.
- List each transitioned property. Never use `transition: all`.
- Do not animate layout properties such as `width`, `height`, `top`, `margin`, or `padding`. Do not animate `filter`, `backdrop-filter`, or `box-shadow`.
- Use CSS transitions. Animate the entrance and exit of `<dialog>` and `popover` elements with `@starting-style` and `transition-behavior: allow-discrete`, not JavaScript.
- Use the Web Animations API only when CSS cannot express an animation. Do not add an animation library.
- Do not run `requestAnimationFrame` loops, and do not drive animations from scroll or resize listeners.
- Apply `will-change` only to an element while it animates.
- Use only the duration and easing tokens. No transition exceeds 160ms.
- Respect `prefers-reduced-motion` through the single guard in `src/styles/base.css`. Do not query it anywhere else.

## Rules

- Do the work at build time, not at runtime. Parse, index, and shape the API dump during generation, in Rust.
- Write markup in the Maud templates in `crates/site`. Tailwind scans `crates/site/src` and `src/client` for class names, so write each class name in full, never built from parts.
- Keep the output deterministic: the same data must build the same files. Sort with `core::js::compare_names` where the site shows names in order.
- Search and the list filters share one scorer, in `crates/search`. Change it there, never in TypeScript.
- Write every filter and search query to the URL, and restore it from the URL when the page loads.
- Import modules directly when possible. Do not create barrel files.
- Split code into small, focused files. Group related files into folders. Do not let one file grow to hold unrelated logic.
- Do not write comments. Write code that is clear enough to need none. Add a comment only when the user explicitly requests unconventional behavior that would confuse a reader.
