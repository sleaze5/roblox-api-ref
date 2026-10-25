# roblox-api-ref

Unofficial Roblox Engine API reference. Search and browse every class, member, enum, and data type, and see what changed in each Roblox Studio build.

The site is static. The build generates it from `https://setup.rbxcdn.com/${version}-Full-API-Dump.json` and from `en-us.json` inside `https://setup.rbxcdn.com/${version}-content-api-docs.zip`.

## Technology stack

- Frontend: SolidJS `2`, TypeScript `7`
- Tooling: Vite+ (Vite, Rolldown, Oxlint, Vitest), Bun (runtime, package manager, and `bun fmt`, its built-in Oxfmt)
- Design: Tailwind CSS `4`, Geist, Geist Mono, Phosphor icons

Solid 2 differs from Solid 1.x. Read `node_modules/solid-js/CHEATSHEET.md` before you write Solid code.

## Task orchestration

Interact with the project only through `bun vp run <task>`. Do not run Vite, Oxlint, Vitest, or `bun fmt` directly, and do not install Node.js or a global `vp`. Bun is the only tool to install. `bun install` installs `vp` from the `vite-plus` package. Run `bun vp run` to list the available tasks.

Tasks are defined in `run.tasks` in `vite.config.ts`. Vite+ runs the tasks in `dependsOn` first and caches task results.

`bunfig.toml` sets `run.bun`, so `bun vp` runs `vp`, and every tool that it starts, on Bun instead of Node.js. In a task command, start a tool with `bun vp <command>`, and run another task with a bare `vp run <task>`. Vite+ runs a bare `vp run` in the same task graph. `bun vp run` inside a task starts a nested runner, which fails.

- `dump:update` reads the LIVE `WindowsStudio64` version from `clientsettings.roblox.com`. Only Studio uploads include the API dump and the API docs. When that build is newer than the last version in `roblox-versions.json`, the task downloads both API dumps and both English API docs into `data/`, writes the changes between them to `diffs/<version>.json`, and appends the build and its publish date to `roblox-versions.json`. When `roblox-versions.json` is empty, the task starts from the Studio build before the live one in [version-history.json](https://github.com/MaximumADHD/Roblox-Client-Tracker/blob/roblox/version-history.json). It skips a new version until its API dump and API docs are on the CDN.
- `deps:update` updates each package in `package.json`, and the Bun version in `devEngines`, to the newest stable release that keeps the same major version, or the same minor version below 1.0.0. It skips deprecated releases. A prerelease version moves along its own line, such as `2.0.0-rc.15` to `2.0.0-rc.16`, or to the stable release of the same version, such as `2.0.0`. Then it runs `bun install` to update `bun.lock`. The `Update dependencies` workflow runs it every Monday, runs `ci`, and opens or updates a pull request from the `deps/update` branch, and starts CI on that branch. Dependabot updates the GitHub Actions versions the same way.
- `dump` downloads the API dump and the English API docs for the last version in `roblox-versions.json` into `data/`, unless they are already downloaded. It reads `en-us.json` out of `content-api-docs.zip` with HTTP range requests instead of downloading the whole archive. It deletes every other dump and API docs file in `data/`.
- `gen` runs `dump`, then generates data into `src/generated/`, including the Updates pages from `diffs/`. It is never cached.
- `dev`, `test`, `test:watch`, `typecheck`, and `check` run `gen` first.
- `format:check` and `format:fix` run `bun fmt` with the `fmt` options in `vite.config.ts`. `check` runs `format:check`, then `vp check --no-fmt`.
- `fix` runs `lint:fix`, `format:fix`, and `check`, in that order.
- `build` runs `fix`, then builds into `dist/`. `preview` runs `build` first.
- `ci` runs `check`, `test`, and a build. It never changes files.

Do not add or remove tasks unless the request requires it. When you add a task, set `cache: false` if the task uses the network, writes source files, or runs a server. Vite+ cannot see the files that a Bun process reads, so give every other new task that runs Bun `sourceInput` in `cache.input`.

## Versions and diffs

- `roblox-versions.json` lists every version that the site has published, oldest first. The last entry is the current version. Each entry names a Studio build by its `version` and `hash`.
- `diffs/<version>.json` holds the changes from the previous entry in `roblox-versions.json` to `<version>`. Commit the diffs and `roblox-versions.json`. Do not commit API dumps.
- `dump:update` writes both files. Do not edit them by hand.
- A diff lists changes in the shape of the API dump: `element` is `Class`, `Member`, `Enum`, or `EnumItem`, and `type` is `Add`, `Remove`, or `Change`. `value` holds the dump entry after the change, or before it for `Remove`. A `Change` also lists each changed dump field with its `before` and `after` values. An added class or enum also gets an `Add` for each of its members or items. A removed class or enum does not list its members or items.
- A diff also lists `descriptions`: each description in `en-us.json` that was added, changed, or removed for a class, member, parameter, return value, enum, or enum item that exists in the new version. `part` is `description`, `parameter`, or `return`, and `before` and `after` hold the HTML, or `null` when there is none. A member or enum item change also holds its dump entry in `value`. Roblox publishes new API docs once per release, so builds of the same release usually have no description changes.

## Hosting and agents

- The site is a static build hosted on Cloudflare Pages at `https://rapi.sleaze.dev/`. Cloudflare's Git integration builds `dist/` and deploys every push to the default branch as production, and every push to another branch as a preview. No GitHub workflow deploys.
- Cloudflare runs `bash scripts/cloudflare-build.sh` with `SKIP_DEPENDENCY_INSTALL=1` and no `BUN_VERSION`, and serves `dist/`. The script installs the exact Bun version in `devEngines` in `package.json` and runs `bun install --frozen-lockfile` and `bun vp run ci`. The build does not precompress files, because Cloudflare compresses responses itself. `package.json` is the only place that names the Bun version.
- `src/site/origin.ts` resolves the origin at build time from `SITE_ORIGIN`, then `CF_PAGES_URL`, then `http://localhost:5173`. Production sets `SITE_ORIGIN` in Cloudflare. Previews use their own `CF_PAGES_URL`. The `build`, `test`, and `ci` tasks list both variables in `cache.env`, because `vp run` passes only listed variables to a cached task. Every absolute URL in pages, Markdown, the sitemap, and JSON-LD uses this origin. Do not hardcode the production domain. Tests compare against `siteOrigin`, not a literal origin.
- Every page has a Markdown version. Its URL is the page URL without the trailing slash, followed by `.md`. The Markdown version of the home page is `llms.txt`, and the Markdown version of the not found page is `404.md`.
- `functions/_middleware.ts` is a Cloudflare Pages Function that negotiates Markdown with `src/edge/negotiate.ts`. A page request that prefers `text/markdown` in its `Accept` header gets the Markdown version from the same URL, or `404.md` with status 404 when the page does not exist. Other page requests get the HTML with a `Link` header that points to the Markdown version. Every negotiated response has `Vary: Accept`. A request that accepts neither HTML nor Markdown gets 406. `bun vp run preview` serves `dist/` through the same negotiation.
- `public/_routes.json` keeps the Function off `/assets/` and the files at the site root, so that those requests stay free static requests. Every page route must run the Function.
- Write absolute links in Markdown, so that the same Markdown works from any URL.
- GitHub holds the source and runs the automation. The `Setup` action installs the Bun version in `devEngines` with `setup-bun`, then runs `bun install --frozen-lockfile`. The CI workflow checks, tests, and builds every push and fork pull request. The hourly release workflow commits each new Roblox version to the default branch, which starts a Cloudflare deploy, and then starts CI, because pushes made with `GITHUB_TOKEN` do not start workflows. The `Update dependencies` workflow and Dependabot keep dependencies current.

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

- Do the work at build time, not at runtime. Parse, index, and shape the API dump during generation.
- Write every filter and search query to the URL, and restore it from the URL when the page loads.
- Import modules directly when possible. Do not create barrel files.
- Split code into small, focused files. Group related files into folders. Do not let one file grow to hold unrelated logic.
- Do not write comments. Write code that is clear enough to need none. Add a comment only when the user explicitly requests unconventional behavior that would confuse a reader.
