export type EmptyState = {
	update: (query: string, matched: number) => void
}

export function createEmptyState(
	form: HTMLFormElement,
	input: HTMLInputElement,
): EmptyState {
	const message = document.createElement("div")
	message.hidden = true
	message.className =
		form.closest("#sidebar") === null
			? "flex flex-col items-start gap-3 py-6 text-base text-fg-muted"
			: "flex flex-col items-start gap-3 px-2 py-4 text-fg-muted"
	const text = document.createElement("p")
	const query = document.createElement("code")
	query.className = "text-fg"
	const clear = document.createElement("button")
	clear.type = "button"
	clear.className = "button"
	clear.textContent = "Clear filter"
	clear.addEventListener("click", () => {
		input.value = ""
		input.dispatchEvent(new Event("input", { bubbles: true }))
		input.focus()
	})
	text.append("Nothing matches ", query, ". Check the spelling, or clear the filter.")
	message.append(text, clear)
	form.after(message)
	return {
		update(value, matched) {
			message.hidden = value === "" || matched > 0
			query.textContent = value
		},
	}
}
