export const homeMarkdownPath = "/llms.txt"

export const notFoundMarkdownPath = "/404.md"

export function markdownPath(pathname: string): string | undefined {
	if (pathname === "/") {
		return homeMarkdownPath
	}
	const page = pathname.replace(/\/$/, "")
	if (page !== pathname) {
		return `${page}.md`
	}
	return page.slice(page.lastIndexOf("/")).includes(".") ? undefined : `${page}.md`
}
