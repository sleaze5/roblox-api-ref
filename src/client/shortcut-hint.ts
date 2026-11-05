function isApple(): boolean {
	return /Mac|iPhone|iPad|iPod/.test(navigator.platform)
}

export function initShortcutHint(): void {
	if (!isApple()) {
		return
	}
	for (const hint of document.querySelectorAll<HTMLElement>("[data-shortcut]")) {
		hint.textContent = "⌘ K"
		hint.closest("[aria-keyshortcuts]")?.setAttribute(
			"aria-keyshortcuts",
			"Meta+K /",
		)
	}
}
