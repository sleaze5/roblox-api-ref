import type { DocLink } from "../../api/doc-types.ts"
import {
	classHref,
	enumHref,
	enumItemHref,
	memberHref,
	typeHref,
} from "../../api/routes.ts"

export function docLinkHref(link: DocLink): string {
	switch (link.kind) {
		case "class":
			return classHref(link.name)
		case "member":
			return memberHref(link.owner, link.member, link.name)
		case "enum":
			return enumHref(link.name)
		case "enumItem":
			return enumItemHref(link.owner, link.name)
		case "type":
			return typeHref(link.name)
		case "external":
			return link.url
	}
}

export type DocCodeTone = "type" | "constant" | "function" | "member"

export function docCodeTone(link: DocLink): DocCodeTone {
	switch (link.kind) {
		case "enumItem":
			return "constant"
		case "member":
			return link.member === "Method" ? "function" : "member"
		default:
			return "type"
	}
}
