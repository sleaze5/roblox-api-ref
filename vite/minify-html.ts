import { minify } from "@minify-html/node"
import { minify as minifyScript } from "terser"

const inlineScriptPattern = /<script>([\s\S]*?)<\/script>/g
const jsonScriptPattern = /(<script type="speculationrules">)([\s\S]*?)(<\/script>)/g

async function minifyInlineScript(source: string): Promise<string> {
	const result = await minifyScript(source, {
		ecma: 2020,
		compress: { passes: 3, unsafe: true },
		mangle: true,
		format: { comments: false },
	})
	return result.code ?? source
}

export async function minifyTemplateScripts(template: string): Promise<string> {
	const scripts = [...template.matchAll(inlineScriptPattern)]
	const minified = await Promise.all(
		scripts.map((match) => minifyInlineScript(match[1] ?? "")),
	)
	let index = 0
	return template
		.replace(
			inlineScriptPattern,
			() => `<script>${minified[index++] ?? ""}</script>`,
		)
		.replace(
			jsonScriptPattern,
			(_, open: string, json: string, close: string) =>
				`${open}${JSON.stringify(JSON.parse(json))}${close}`,
		)
}

export function minifyHtml(html: string): string {
	return minify(Buffer.from(html), {
		keep_closing_tags: false,
		keep_comments: false,
		keep_html_and_head_opening_tags: false,
		keep_input_type_text_attr: false,
		minify_css: true,
		minify_js: false,
		remove_bangs: true,
		remove_processing_instructions: true,
	}).toString()
}
