import tailwindcss from "@tailwindcss/vite"
import { defineConfig } from "vite-plus"
import { pages } from "./vite/pages.ts"

const ignorePatterns = [
	"dist/**",
	"node_modules/**",
	"data/**",
	"diffs/**",
	"src/generated/**",
	"AGENTS_EXPLORE/**",
	"target/**",
]
const stylesheet = "src/styles/app.css"
const sourceInput = [
	{ auto: true },
	"**",
	"!.git/**",
	"!node_modules/**",
	"!dist/**",
	"!data/**",
	"!target/**",
]
const originCache = { env: ["SITE_ORIGIN", "CF_PAGES_URL"], input: sourceInput }

export default defineConfig({
	plugins: [tailwindcss(), pages()],
	css: { transformer: "lightningcss" },
	build: {
		target: "esnext",
		minify: "oxc",
		cssMinify: "lightningcss",
		modulePreload: { polyfill: false },
		reportCompressedSize: false,
	},
	server: { host: "127.0.0.1", port: 5173, strictPort: true },

	test: {
		include: [
			"src/**/*.test.{ts,tsx}",
			"scripts/**/*.test.ts",
			"vite/**/*.test.ts",
		],
		environment: "node",
		passWithNoTests: true,
	},

	fmt: {
		printWidth: 88,
		tabWidth: 4,
		useTabs: true,
		semi: false,
		singleQuote: false,
		jsxSingleQuote: false,
		trailingComma: "all",
		endOfLine: "lf",
		arrowParens: "always",
		bracketSameLine: true,
		bracketSpacing: true,
		proseWrap: "never",
		htmlWhitespaceSensitivity: "css",
		objectWrap: "preserve",
		quoteProps: "consistent",
		singleAttributePerLine: false,
		sortPackageJson: false,
		sortTailwindcss: { stylesheet: `./${stylesheet}` },
		ignorePatterns,
	},

	lint: {
		categories: {
			correctness: "error",
			perf: "warn",
			suspicious: "error",
		},
		plugins: ["eslint", "typescript", "oxc", "import", "promise", "vitest"],
		jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
		env: { browser: true, builtin: true },
		ignorePatterns,
		options: { typeAware: true, typeCheck: true },
		rules: {
			"complexity": ["error", 15],
			"import/no-unassigned-import": "off",
			"typescript/no-unsafe-type-assertion": "off",
			"typescript/consistent-return": "off",
			"vite-plus/prefer-vite-plus-imports": "error",
			"no-restricted-imports": [
				"error",
				{
					paths: [
						{
							name: "@phosphor-icons/core",
							message:
								"Import icons directly from @phosphor-icons/core/<weight>/<name>.svg.",
						},
					],
				},
			],
		},
		overrides: [
			{
				files: ["vite.config.ts", "vite/**", "scripts/**"],
				env: { browser: false, node: true },
			},
		],
	},

	run: {
		tasks: {
			"setup": {
				command: ["bun install", "vp run gen"],
				cache: false,
			},
			"dump:update": {
				command: "cargo rapi dump-update",
				cache: false,
			},
			"deps:update": {
				command: ["bun scripts/deps/update.ts", "bun install"],
				cache: false,
			},
			"dump": {
				command: "cargo rapi dump",
				cache: false,
			},
			"gen": {
				command: ["cargo rapi gen", "bun scripts/fonts.ts"],
				dependsOn: ["dump"],
				cache: false,
			},
			"dev": {
				command: "bun vp dev",
				dependsOn: ["gen"],
				cache: false,
			},
			"build": {
				command: ["vp run fix", "bun vp build"],
				cache: originCache,
			},
			"preview": {
				command: "bun vp preview",
				dependsOn: ["build"],
				cache: false,
			},
			"test": {
				command: ["cargo test --workspace --quiet", "bun vp test run"],
				dependsOn: ["gen"],
				cache: originCache,
			},
			"test:watch": {
				command: "bun vp test",
				dependsOn: ["gen"],
				cache: false,
			},
			"test:browser": {
				command: "bun test --timeout 60000 tests/browser",
				dependsOn: ["build"],
				cache: false,
			},
			"bench": {
				command: "bun scripts/bench.ts",
				dependsOn: ["gen"],
				cache: false,
			},
			"format:check": {
				command: ["cargo fmt --all --check", "bun fmt --check"],
				cache: { input: sourceInput },
			},
			"format:fix": {
				command: ["cargo fmt --all", "bun fmt"],
				cache: false,
			},
			"clippy":
				"cargo clippy --workspace --all-targets --quiet -- --deny warnings",
			"lint:check": {
				command: ["vp run clippy", "bun vp lint --deny-warnings"],
			},
			"lint:fix": {
				command: [
					"cargo clippy --workspace --all-targets --quiet --fix --allow-dirty --allow-staged -- --deny warnings",
					"bun vp lint --fix --deny-warnings",
				],
				cache: false,
			},
			"fix": {
				command: ["vp run lint:fix", "vp run format:fix", "vp run check"],
				cache: false,
			},
			"typecheck": {
				command: "bun vp check --no-fmt --no-lint",
				dependsOn: ["gen"],
			},
			"check": {
				command: [
					"vp run format:check",
					"vp run clippy",
					"bun vp check --no-fmt",
				],
				dependsOn: ["gen"],
			},
			"ci": {
				command: ["vp run check", "vp run test", "bun vp build"],
				cache: originCache,
			},
		},
	},
})
