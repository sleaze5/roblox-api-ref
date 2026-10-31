import { initCodeSamples } from "./code-sample/code-sample.ts"
import { initCopyLinks } from "./copy-link.ts"
import { initDrawer } from "./drawer.ts"
import { initListFilter } from "./filters/list-filter.ts"
import { initMemberFilter } from "./members/member-filter.ts"
import { initScrollSpy } from "./rail/scroll-spy.ts"
import { initSearchTrigger } from "./search/search-trigger.ts"
import { initTooltips } from "./tooltip/tooltip.ts"
import { initStickyRows } from "./tree/sticky-rows.ts"

initSearchTrigger()
initTooltips()
initCopyLinks()
initCodeSamples()
initDrawer()
initStickyRows()

for (const form of document.querySelectorAll<HTMLFormElement>(
	"form[data-member-filter]",
)) {
	initMemberFilter(form)
}

for (const rail of document.querySelectorAll<HTMLElement>("[data-rail]")) {
	initScrollSpy(rail)
}

for (const form of document.querySelectorAll<HTMLFormElement>(
	"form[data-list-filter]",
)) {
	initListFilter(form)
}
