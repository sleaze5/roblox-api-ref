import { describe, expect, test } from "vite-plus/test"
import { matchPositions, scoreName } from "./score.ts"

function score(query: string, name: string): number {
	return scoreName(query, name, name.toLowerCase())
}

describe("scoreName", () => {
	test("ranks exact, prefix, word, substring, and fuzzy matches in order", () => {
		const exact = score("touched", "Touched")
		const prefix = score("touch", "Touched")
		const word = score("touch", "CanTouch")
		const substring = score("ouch", "Touched")
		const fuzzy = score("gtp", "GetTouchingParts")
		expect(exact).toBeGreaterThan(prefix)
		expect(prefix).toBeGreaterThan(word)
		expect(word).toBeGreaterThan(substring)
		expect(substring).toBeGreaterThan(fuzzy)
		expect(fuzzy).toBeGreaterThan(0)
	})

	test("rejects letters spread across a long name", () => {
		expect(score("material", "TextScraperClientMessageWithPlayerSignal")).toBe(0)
	})

	test("returns zero when the letters are not in order", () => {
		expect(score("xyz", "Touched")).toBe(0)
	})
})

describe("matchPositions", () => {
	test("prefers word boundaries for acronyms", () => {
		expect(matchPositions("gtp", "GetTouchingParts")).toEqual([0, 3, 11])
	})
})
