import type { FilterTag, FilterTagGroup } from "../../api/member-filter-tags.ts"
import { Icon } from "../Icon.tsx"

const menuId = "member-filter-menu"

function TagRow(props: { tag: FilterTag }) {
	return (
		<li data-tag-row="" class="menu-row">
			<span aria-hidden="true" class="min-w-0 flex-1 truncate">
				{props.tag.shortLabel}
			</span>
			<span class="text-xs text-fg-faint">{props.tag.count}</span>
			<label data-tooltip="Require" class="tag-toggle">
				<input
					type="checkbox"
					name="require"
					value={props.tag.key}
					class="sr-only"
				/>
				<span class="sr-only">Require {props.tag.label}</span>
				<Icon name="check-circle" size={16} />
			</label>
			<label data-tooltip="Exclude" class="tag-toggle tag-toggle-exclude">
				<input
					type="checkbox"
					name="exclude"
					value={props.tag.key}
					class="sr-only"
				/>
				<span class="sr-only">Exclude {props.tag.label}</span>
				<Icon name="prohibit" size={16} />
			</label>
		</li>
	)
}

function MatchMode() {
	return (
		<fieldset class="flex items-center justify-between gap-3 px-2">
			<legend class="sr-only">Required tags</legend>
			<span aria-hidden="true" class="text-fg-muted">
				Required tags
			</span>
			<span class="segmented">
				<label>
					<input
						type="radio"
						name="match"
						value="any"
						checked
						class="sr-only"
					/>
					Any
				</label>
				<label>
					<input type="radio" name="match" value="all" class="sr-only" />
					All
				</label>
			</span>
		</fieldset>
	)
}

export function FilterMenu(props: { groups: FilterTagGroup[] }) {
	return (
		<>
			<button
				type="button"
				popovertarget={menuId}
				class="menu-button menu-anchor">
				<Icon name="funnel-simple" size={14} />
				Filter
				<span data-tag-filter-count="" hidden class="count-badge" />
				<Icon name="caret-down" size={14} class="text-fg-faint" />
			</button>
			<div
				id={menuId}
				popover="auto"
				role="dialog"
				aria-label="Filters"
				class="menu">
				<div class="flex flex-col gap-1 border-b border-line p-1 pb-2">
					<label class="menu-row cursor-pointer">
						<span class="min-w-0 flex-1 truncate">Inherited members</span>
						<span class="switch me-1">
							<input
								type="checkbox"
								name="inherited"
								checked
								class="sr-only"
							/>
						</span>
					</label>
					<MatchMode />
				</div>
				<div class="flex max-h-80 flex-col gap-2 overflow-y-auto overscroll-contain p-1">
					{props.groups.map((group) => (
						<fieldset>
							<legend class="menu-label">{group.title}</legend>
							<ul>
								{group.tags.map((tag) => (
									<TagRow tag={tag} />
								))}
							</ul>
						</fieldset>
					))}
				</div>
				<div class="flex items-center justify-between gap-2 border-t border-line p-2">
					<p class="text-xs text-fg-faint">
						Members need any or all required tags, and none of the excluded
						ones.
					</p>
					<button type="button" data-tag-filter-reset="" class="button">
						Reset
					</button>
				</div>
			</div>
		</>
	)
}
