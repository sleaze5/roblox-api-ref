export function announce(message: string): void {
	const region = document.getElementById("announcer")
	if (region === null) {
		return
	}
	region.textContent = ""
	setTimeout(() => {
		region.textContent = message
	}, 50)
}
