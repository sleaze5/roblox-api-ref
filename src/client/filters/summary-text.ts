export function summaryText(shown: number, total: number): string {
	return shown === total ? `${total} total` : `${shown} of ${total}`
}
