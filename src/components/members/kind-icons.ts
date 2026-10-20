import type { MemberKind } from "../../api/types.ts"
import type { IconName } from "../icon-names.ts"

export const kindIcons: Record<MemberKind, IconName> = {
	Property: "sliders-horizontal",
	Method: "function",
	Event: "lightning",
	Callback: "arrow-u-up-left",
}
