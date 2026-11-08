export type EmptyMembers = {
	update: (query: string, shown: number) => void
}

export function createEmptyMembers(
	form: HTMLFormElement,
	onClear: () => void,
): EmptyMembers {
	const message = document.createElement("div")
	message.hidden = true
	message.className = "flex flex-col items-start gap-3 py-6 text-base text-fg-muted"
	const text = document.createElement("p")
	const clear = document.createElement("button")
	clear.type = "button"
	clear.className = "button"
	clear.textContent = "Clear filters"
	clear.addEventListener("click", () => {
		onClear()
		form.querySelector<HTMLInputElement>("input[name=text]")?.focus()
	})
	message.append(text, clear)
	form.after(message)
	return {
		update(query, shown) {
			message.hidden = shown > 0
			if (query === "") {
				text.textContent =
					"No members match the filters. Adjust them, or clear the filters."
				return
			}
			const code = document.createElement("code")
			code.className = "text-fg"
			code.textContent = query
			text.replaceChildren(
				"Nothing matches ",
				code,
				". Check the spelling, or clear the filters.",
			)
		},
	}
}
