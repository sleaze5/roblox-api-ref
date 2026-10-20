import solid from "@solidjs/vite-plugin"
import tailwindcss from "@tailwindcss/vite"
import { defineConfig } from "vite-plus"
import { pages } from "./vite/pages.ts"

const ignorePatterns = [
	"dist/**",
	"node_modules/**",
	"data/**",
	"src/generated/**",
	"AGENTS_EXPLORE/**",
]
const stylesheet = "src/styles/app.css"
const siteEnv = { env: ["VITE_SITE_*"] }

export default defineConfig({
	base: process.env["VITE_SITE_BASE"] ?? "/",
	plugins: [
		solid({ compiler: "native", ssr: true, solid: { hydratable: false } }),
		tailwindcss(),
		pages(),
	],
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
		include: ["src/**/*.test.{ts,tsx}", "scripts/**/*.test.ts"],
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
		jsPlugins: [
			{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" },
			{
				name: "better-tailwindcss",
				specifier: "eslint-plugin-better-tailwindcss",
			},
		],
		settings: { "better-tailwindcss": { entryPoint: stylesheet } },
		env: { browser: true, builtin: true },
		ignorePatterns,
		options: { typeAware: true, typeCheck: true },
		rules: {
			"complexity": ["error", 15],
			"import/no-unassigned-import": "off",
			"typescript/no-unsafe-type-assertion": "off",
			"typescript/consistent-return": "off",
			"vite-plus/prefer-vite-plus-imports": "error",
			"better-tailwindcss/no-unknown-classes": "error",
			"better-tailwindcss/no-conflicting-classes": "error",
			"better-tailwindcss/no-duplicate-classes": "error",
			"better-tailwindcss/no-restricted-classes": [
				"error",
				{
					restrict: [
						{
							pattern: "\\[.+\\]",
							message:
								"Use a design token from src/styles/theme.css instead of an arbitrary value.",
						},
					],
				},
			],
			"no-restricted-imports": [
				"error",
				{
					paths: [
						{
							name: "@phosphor-icons/core",
							message:
								"Import icons directly from @phosphor-icons/core/<weight>/<name>.svg.",
						},
						{
							name: "solid-js/web",
							message: "Solid 2 moved the DOM renderer to @solidjs/web.",
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
			"dump:update": {
				command: "bun scripts/dump/update.ts",
				cache: false,
			},
			"dump": {
				command: "bun scripts/dump/download.ts",
				cache: false,
			},
			"gen": {
				command: "bun scripts/gen/main.ts",
				dependsOn: ["dump"],
				cache: false,
			},
			"dev": {
				command: "bun --bun vp dev",
				dependsOn: ["gen"],
				cache: false,
			},
			"build": {
				command: ["vp run fix", "bun --bun vp build"],
				cache: siteEnv,
			},
			"preview": {
				command: "bun --bun vp preview",
				dependsOn: ["build"],
				cache: false,
			},
			"test": {
				command: "bun --bun vp test run",
				dependsOn: ["gen"],
			},
			"test:watch": {
				command: "bun --bun vp test",
				dependsOn: ["gen"],
				cache: false,
			},
			"format:check": "bun --bun vp fmt --check",
			"format:fix": {
				command: "bun --bun vp fmt",
				cache: false,
			},
			"lint:check": "bun --bun vp lint --deny-warnings",
			"lint:fix": {
				command: "bun --bun vp lint --fix --deny-warnings",
				cache: false,
			},
			"fix": {
				command: ["vp run lint:fix", "vp run format:fix", "vp run check"],
				cache: false,
			},
			"typecheck": {
				command: "bun --bun vp check --no-fmt --no-lint",
				dependsOn: ["gen"],
			},
			"check": {
				command: "bun --bun vp check",
				dependsOn: ["gen"],
			},
			"ci": {
				command: ["vp run check", "vp run test", "bun --bun vp build"],
				cache: siteEnv,
			},
		},
	},
})
