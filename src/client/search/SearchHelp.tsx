import { uploadName } from "../../api/release.ts"
import type { SearchUpdate } from "../../search/index-format.ts"

const examples = [
	{
		query: "Part.",
		description: "Every member of Part, including inherited members",
	},
	{ query: "Humanoid.Move", description: "Members of Humanoid that match Move" },
	{ query: ".Touched", description: "Members named Touched on any class" },
	{ query: "gtp", description: "Acronyms also match, such as GetTouchingParts" },
	{
		query: "is:enum Material",
		description:
			"Only enums. Other kinds: class, type, property, method, event, callback, item, and member",
	},
	{
		query: "tag:yields !tag:deprecated",
		description: "Members that yield and are not deprecated",
	},
	{
		query: "Material",
		description: "The Material enum, and every member that uses it",
	},
	{
		query: "Enum.Material.Plas",
		description: "Items of the Material enum that match Plas",
	},
	{
		query: "type:Color3 is:property",
		description: "Properties whose type is Color3",
	},
	{
		query: "uses:Enum.KeyCode",
		description: "Members that use KeyCode as a value, parameter, or return value",
	},
	{ query: "access:game Get", description: "Members that game scripts can use" },
	{ query: '"Name"', description: "Exact names only" },
]

function updateExample(update: SearchUpdate | undefined) {
	return update === undefined
		? []
		: [
				{
					query: uploadName(update.hash),
					description:
						"The update for a Studio version, by its hash or version number",
				},
			]
}

export function SearchHelp(props: {
	onPick: (query: string) => void
	update?: SearchUpdate | undefined
}) {
	return (
		<div class="flex flex-col gap-1 p-1">
			<p class="px-1 pb-1 text-xs font-medium text-fg-muted">Search syntax</p>
			{[...examples, ...updateExample(props.update)].map((example) => (
				<button
					type="button"
					onClick={() => props.onPick(example.query)}
					class="flex items-baseline gap-3 rounded-sm px-2 py-1.5 text-left motion-colors hover:bg-hover">
					<code class="shrink-0 text-fg-bright">{example.query}</code>
					<span class="text-fg-muted">{example.description}</span>
				</button>
			))}
		</div>
	)
}
