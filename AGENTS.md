# roblox-api-ref

A statically generated, interactive Roblox Engine API reference site. The build generates it from `https://setup.rbxcdn.com/${version}-Full-API-Dump.json` and from `en-us.json` inside `https://setup.rbxcdn.com/${version}-content-api-docs.zip`.

## Technology stack

- Frontend: SolidJS `2`, TypeScript `7`
- Tooling: Vite+ (Vite, Rolldown, Oxlint, Oxfmt, Vitest), Bun (runtime and package manager)
- Design: Tailwind CSS `4`, Geist, Geist Mono, Phosphor icons

Solid 2 differs from Solid 1.x. Read `node_modules/solid-js/CHEATSHEET.md` before you write Solid code.

## Task orchestration

Interact with the project only through `vp run <task>`. Do not run Bun, Vite, Oxlint, Oxfmt, or Vitest directly. Run `vp run` to list the available tasks.

Tasks are defined in `run.tasks` in `vite.config.ts`. Vite+ runs the tasks in `dependsOn` first and caches task results.

- `dump:update` saves the latest LIVE Roblox Studio version to `roblox-version.json`. It does not download anything.
- `dump` downloads the API dump and the English API docs for the version in `roblox-version.json` into `data/`, unless they are already downloaded. It reads `en-us.json` out of `content-api-docs.zip` with HTTP range requests instead of downloading the whole archive. It deletes every other dump and API docs file in `data/`.
- `gen` runs `dump`, then generates data into `src/generated/`. It is never cached.
- `dev`, `test`, `test:watch`, `typecheck`, and `check` run `gen` first.
- `fix` runs `lint:fix`, `format:fix`, and `check`, in that order.
- `build` runs `fix`, then builds into `dist/`. `preview` runs `build` first.
- `ci` runs `check`, `test`, and a build. It never changes files.

Do not add or remove tasks unless the request requires it. When you add a task, set `cache: false` if the task uses the network, writes source files, or runs a server.

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
