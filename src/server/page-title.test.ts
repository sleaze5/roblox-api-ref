import { expect, test } from "vite-plus/test"
import { aboutHref, classHref, enumHref, typeHref } from "../api/routes.ts"
import { loadApi } from "./load-api.ts"
import { loadUpdates } from "./load-updates.ts"
import { matchRoute } from "./match-route.ts"
import { pageTitle } from "./page-title.ts"

const api = loadApi()
const updates = loadUpdates()

function titleOf(path: string): string {
	return pageTitle(matchRoute(api, updates, path))
}

test("names each page before the site name", () => {
	expect(titleOf("/")).toBe("Roblox API Reference")
	expect(titleOf(classHref("Part"))).toBe("Part | Roblox API Reference")
	expect(titleOf(enumHref("Material"))).toBe("Enum.Material | Roblox API Reference")
	expect(titleOf(typeHref("CFrame"))).toBe("CFrame | Roblox API Reference")
	expect(titleOf("/index/")).toBe("Index | Roblox API Reference")
	expect(titleOf(aboutHref())).toBe("About | Roblox API Reference")
	expect(titleOf("/nope/")).toBe("Page not found | Roblox API Reference")
})
