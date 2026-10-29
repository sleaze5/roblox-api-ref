# Design

This file is the authoritative specification for visual design, component styling, interaction states, accessibility, interface text, and motion. When another document conflicts with it on these topics, follow this file.

The site reads like a dark code editor: near-black neutral surfaces, light text, thin borders, and dense, precise Luau lines. Code is the only colored thing on the page. Everything is instant.

When this file does not cover a case, copy the closest existing pattern on the site. If no pattern fits, follow the principles below and name the new pattern in the change summary.

## Principles

1. Code is the only color. The interface is grayscale. Hue appears only in code (identifiers, types, values), in the few semantic tags, and in Studio class icons. A reader finds code by its color without reading.
2. Identifiers are monospace, prose is sans. Every API name, type, signature, and value uses Geist Mono, at every size. Every sentence, label, and control uses Geist. Do not use monospace for anything that is not code.
3. Write members as Luau. Every member reads like a line of a Luau type definition, such as `Terrain.Decoration: boolean = false`. Use Roblox's own terms: class, data type, enum, property, method, event, callback.
4. Instant. Pages are prerendered static HTML. Nothing waits for JavaScript, a fade, or a spinner before the reader can read. Motion only answers an action.
5. Readable without JavaScript. Every page renders, links, and expands without JavaScript. JavaScript adds search, filters, and conveniences such as copying links.
6. The address is the state. Every filter and search query is written to the URL. A copied address opens the same view.

## Sources of truth

- `src/styles/theme.css` defines every color, font, text size, spacing step, radius, shadow, layout dimension, duration, and easing as Tailwind theme tokens. Use their utilities. Arbitrary values such as `p-[13px]` fail the lint task.
- Add a token for a value that repeats or that defines the design. Put a repeated group of declarations in an `@utility` in `src/styles/`.
- `src/styles/base.css` holds global element styles, the focus ring, and the reduced-motion guard. Define the guard only there.
- When TypeScript needs a token value, read it from the computed style. Do not copy token values into TypeScript.

## Building blocks

Build from these utilities before you write new styles. If one lacks a variant you need, extend it. Do not copy its declarations into a class string.

- `field`: a 32px text input. Add `field-icon` when a 14px icon sits inside it at `left-3`, and `field-count` when a filter count sits inside its right edge.
- `menu-button` with `menu-anchor`, and `menu`: a 32px button that opens an anchored popover menu. Build menu rows with `menu-row` and `menu-label`, on/off settings with a trailing `switch`, two-way choices with `segmented`, per-row icon toggles with `tag-toggle`, and counts with `count-badge`.
- `chip`: a 28px toggle that wraps a visually hidden checkbox.
- `button`: a 28px bordered secondary button.
- `picker`: the surface of a floating list.
- `tag-chip`, `tag-warning`, and `tag-danger`: tags. `kbd`: a keyboard hint.
- `tooltip`: the single tooltip surface, driven by `data-tooltip` through `src/client/tooltip/`.

## Browser UI

Everything the reader sees must look designed for this site. No browser-drawn control may appear unstyled.

- Do not use the `title` attribute. Put the accessible name in `aria-label` or visible text, and the hint in `data-tooltip`. Tooltips show after 500ms on hover and at once on keyboard focus, and close on `Escape`, scroll, and pointer down.
- Explain each fact label with a tooltip of one short sentence that ends with a period, such as "Classes that inherit directly from this class." A label with a tooltip has a dotted `--color-line-strong` underline and the help cursor. The class page rail explains its counts the same way.
- When a link's text is cut off with an ellipsis, such as a long name in the sidebar, the rail, or an Index list, its tooltip shows the full text. Do not add `data-tooltip` for this. The tooltip script detects the cut-off text.
- Do not use `<datalist>`, visible native checkboxes, or unstyled `<select>` elements. Use the building blocks above. Use a `menu` instead of a `<select>`.
- Do not use `alert()`, `confirm()`, or `prompt()`. Announce short results, such as "Link copied", through the `#announcer` live region with `announce()`.
- Style scrollbars, text selection, focus rings, and placeholders through `base.css`.

## Color

The palette is a neutral near-black, based on RobloxAccountManager and shifted one step darker. Do not tint it warm or cool. Do not add blue to the grays.

Surfaces, from lowest to highest:

- `--color-canvas` (`#0e0e0e`): the page and the main content column
- `--color-surface` (`#121212`): the header and the sidebar
- `--color-raised` (`#181818`): inputs
- `--color-overlay` (`#151515`): menus, popovers, tooltips, and the search dialog
- `--color-hover` (`rgb(255 255 255 / 6%)`): hover fill on rows and controls
- `--color-active` (`rgb(255 255 255 / 9%)`): selected row fill
- `--color-scrim` (`rgb(0 0 0 / 72%)`): backdrop behind the search dialog and the mobile sidebar

Lines:

- `--color-line-subtle` (`rgb(255 255 255 / 6%)`): tree guides and separators
- `--color-line` (`#262626`): region borders, control borders, and tags
- `--color-line-strong` (`#363636`): floating surfaces, focused inputs, and selected controls
- `--color-focus` (`rgb(255 255 255 / 40%)`): the focus ring

Text:

- `--color-fg` (`#cccccc`): body text, punctuation in code, and headings
- `--color-fg-muted` (`#a3a3a3`): metadata, secondary labels, and icons
- `--color-fg-faint` (`#8a8a8a`): placeholders, counts, and deprecated names in trees
- `--color-fg-bright` (`#ffffff`): page names, member names, and selected text

`--color-fg-faint` meets 4.5:1 on every surface and on `--color-hover` fills. On `--color-active` fills, use `--color-fg-muted` or a stronger color.

Code uses the GitHub Dark syntax colors:

- `--color-code-type` (`#ffa657`): class, data type, and enum names, and Luau primitive types such as `string`, `number`, `boolean`, and `any`
- `--color-code-type-muted` (the type color mixed with black at 67.647%): deprecated enum names in the enum listing. It keeps their hue and dims them by the same proportion as `--color-fg` to `--color-fg-faint`. Restore the type color on hover and keyboard focus.
- `--color-code-constant` (`#79c0ff`): numbers, booleans, `nil`, and enum items
- `--color-code-string` (`#a5d6ff`): string values
- `--color-code-function` (`#d2a8ff`): method names and constructors such as `new`

The class prefix of a member signature, such as `Part.` in `Part.Anchored`, names `self`. It uses `--color-fg-muted`, not `--color-code-type`. An enum item is written like an enum value in a signature, such as `Enum.ActuatorRelativeTo.Attachment0 = 0`: `Enum.` in `--color-fg`, the enum name in `--color-code-type`, and the item name and value in `--color-code-constant`.

Property, event, and callback names use `--color-fg-bright` at weight 500. Parameter names and punctuation use `--color-fg`. Use code colors only inside code. Do not use them for borders or interface text. Class names in breadcrumbs and inherited group headers are code and use `--color-code-type`.

Semantic:

- `--color-warning` (`#d2a15a`): the Deprecated tag
- `--color-danger` (`#d9675e`): security tags for members that only Roblox can use, and the `−` mark of a removed line on Updates pages
- `--color-success` (`#66a978`): confirmation, such as a copied link, and the `+` mark of an added line on Updates pages

Use semantic colors only for text, icons, 1px borders, and the thin bars and legend swatches of a chart. Do not fill large areas with them.

## Typography

- Use Geist for interface text and prose, and Geist Mono for code. Both are self-hosted variable WOFF2 files in `src/assets/fonts`, declared in `src/styles/fonts.css`. Do not load fonts from a CDN.
- `src/styles/font-fallbacks.css` sizes a local fallback to each font's metrics, Arial for Geist and Courier New for Geist Mono, so a page that renders before the fonts arrive looks almost the same. The fonts use `font-display: optional`: they are preloaded, and when one misses the first paint, that page keeps the fallback instead of reflowing its text, and the next page uses the cached font. Do not size anything in `ch`, which changes with the font: `--container-prose` is 72 Geist zeros written in `em`.
- Create hierarchy with size, weight, and color. Do not use all caps, letter-spaced labels, or italics for emphasis.

Use this scale. Do not use other sizes.

- `--text-xs`: 12px text with a 16px line height, for tags, keyboard hints, and property category headings
- `--text-sm`: 13px text with a 20px line height, for interface text, navigation, all code, and parameter and return descriptions
- `--text-base`: 15px text with a 24px line height, for prose, such as class and member descriptions
- `--text-lg`: 18px text with a 24px line height, for section headings, such as Properties, Methods, Events, and Callbacks
- `--text-xl`: 22px text with a 28px line height, for page titles on index and search pages
- `--text-2xl`: 28px text with a 32px line height, for the class, data type, or enum name at the top of its page

- Use weight 400 for prose, 500 for controls, navigation, and member names, and 600 for headings and the page name. Do not use other weights.
- Set `--text-xl` and `--text-2xl` with `-0.015em` tracking. Use the default tracking for every other size.
- The page name is `--text-2xl`, weight 600, in `--color-fg-bright`, after a 28px icon in `--color-fg-muted`. Name a class, enum, data type, or facet in Geist Mono; name a listing in Geist. It is the one large typographic element on the page.
- Keep prose lines at 72 characters or fewer.
- Use 13px for all code, including inline code inside 15px prose. Inline code gets a `--color-raised` background, `--radius-sm`, and 4px horizontal padding.

## Icons

- Use Phosphor icons in the `regular` weight. Icons are white or gray: they take the text color of their element through `currentColor`.
- Add an icon to the `Icon` enum and to `Icon::ALL` in `crates/core/src/icons.rs`. The `gen` task builds `src/generated/icons.svg`, a sprite that the `icon` and `sprite_icon` helpers in `crates/site/src/html.rs` reference with `<use>`. The search dialog references the same sprite.
- For an icon that repeats on many rows, such as tree nodes and member actions, also add it to `Icon::MASKS` and draw it as a CSS mask. A mask adds no markup to each row.
- Use 16px icons by default, 14px icons next to 12px and 13px text, and 20px icons in section headings.
- Use one icon for one meaning, in every place:

- Class: the class's Studio icon (see [Class icons](#class-icons)), and `cube` for the Classes kind and for a class without an icon
- Data type: `brackets-curly`
- Enum: `list-bullets`
- Enum item: `hash`
- Property: `sliders-horizontal`
- Method: `function`
- Event: `lightning`
- Callback: `arrow-u-up-left`
- Index: `books`
- Update: `git-diff`
- Statistics: `chart-bar`
- About: `info`
- Contact: `envelope-simple`
- Privacy policy: `lock-simple`
- Tag: `tag`
- Security level: `shield`
- Capability: `puzzle-piece`
- Thread safety: `git-fork`
- Memory category: `memory`
- Property category: `folders`
- Search: `magnifying-glass`
- Loading: `circle-notch`
- Filter menu: `funnel-simple`
- Copy a link: `link-simple`
- Copied: `check`
- History: `clock-counter-clockwise`
- Open on Creator Hub: `arrow-square-out`
- Disclosure: `caret-right`
- Close: `x`

- Give decorative icons and separators such as `›` `aria-hidden="true"`. Give every icon-only control an accessible name.

### Class icons

- Every class that exists in the current build shows its Roblox Studio Explorer icon next to its name: on its page title, in the class tree, in Index rows, in search results, and in Updates headings. The icons keep their Studio colors; they are the only icons that are not white or gray.
- The site has a single dark theme, so it uses Studio's dark icons.
- A class without its own icon uses the icon of its nearest superclass that has one, then Studio's generic `Class` icon. Removed classes, which have no page, keep `cube`.
- The `gen` task packs the icons into one sprite, `src/generated/class-icons.png`. An element shows a class icon with the `class-icon` utility and the class's sprite cell in an inline `--icon` style. A `kind-icon` with `data-kind="Class"` and an `--icon` style, and a class tree link with an `--icon` style, switch to the sprite on their own, so rows add no markup beyond the style.

## Spacing and geometry

- Use Tailwind's spacing scale on the 4px `--spacing` base, such as `gap-2` (8px) and `px-3` (12px).
- Use `gap` for rows and columns instead of margins.
- Use 24px controls for compact icon actions, 28px controls for buttons, chips, and tree rows, and 32px controls for the search field and group headers.

Use the radius scale:

- `--radius-xs` (4px): tags and keyboard hints
- `--radius-sm` (6px): buttons, inputs, tree rows, tooltips, and group headers
- `--radius-md` (8px): the home search field and browse links
- `--radius-lg` (10px): menus, popovers, and the search dialog

Keep nested curves concentric: the inner radius equals the outer radius minus the inset. Do not use pills.

- Separate regions with a background step and a 1px border. Use `--shadow-float` only on floating surfaces. Do not put shadows on content.

## Layout

- The header is 48px tall on `--color-surface`. The site name and the section links (Classes, Enums, Index, Updates, and Statistics) sit on the left, the search trigger in the middle, and the build version on the right. From 1024px to 1279px, where the build version is hidden, the search trigger moves to the right edge so the section links have room. Member filters stay in the content column. Write the build version as `version-<hash> (<version>)`, such as `version-7aaa6838d36c4448 (0.744.0.7441077)`, in Geist Mono `--text-xs` and `--color-fg-faint`.
- The sidebar is 264px wide on `--color-surface`. It holds a filter field and the full index of its section: the whole class tree on the Classes listing and class pages, every enum on the Enums listing and enum pages, and the whole index on every Index page. Listings and detail pages of a section share the same sidebar, so moving between them never shifts the content column. The current page is selected, its ancestors are open, and the sidebar starts scrolled to it. Only the home page, the Statistics page, the site pages, and the not found page have no sidebar.
- In the class tree of the sidebar, the row of each open class sticks below the filter field while its subclasses scroll, so the stuck rows show the inheritance path of the rows below them, like the sticky scroll of a code editor's file tree. A row sticks one row lower for each level of depth. Clicking a stuck row scrolls the tree to that class instead of opening its page or collapsing it.
- The content column is on `--color-canvas`. Its maximum width is 760px. The page always reserves the scrollbar gutter, so short and long pages put the column in the same place.
- The right rail is 208px wide. It lists the sections of the current page. On a class page, each section shows the number of members that the class defines, not the total that its section heading shows, and lists those members in Geist Mono and in the same order as the page, linking to their anchors. It does not list inherited members. The rail follows the reader: the section in view is active and shows its member list, the other sections collapse their lists, and the member at the top of the view is highlighted with a `--color-active` fill and `--color-fg-bright` text. Without JavaScript, every list is shown. Until the rail script picks the active section, the first section shows its list. Anchor jumps land below the sticky header and the sticky filter bar.
- Nothing moves once it is painted, and nothing waits for the end of the page. The rail is fixed to the right edge of the window and comes before `main` in the HTML, so it paints with the first bytes of a long page such as Updates, and the content column reserves its width. The skip link, the first focusable element, jumps past it to the content.
- Every filter field shows its count in the prerendered HTML. The page render counts the rows, tree nodes, change lines, or members that the filter script counts, so the number is there before any script runs, and the script only updates it as the reader types. Do not use `content-visibility: auto`: its estimated heights make the page grow and the scrollbar jump while the reader scrolls.
- Keep the right rail on reference and listing pages even when there is only one section. The Classes, Enums, and individual Index listings each have a single rail link to their list. Section count never changes the desktop column layout.
- Every scroll area that sits beside or over the page (the sidebar and drawer, the right rail, menus, pickers, and search results) sets `overscroll-behavior: contain`, so reaching its end never scrolls the page behind it.
- Align all content to the left. Do not center text, headings, or sections.
- At 1280px and wider, show all three columns. From 1024px to 1279px, hide the right rail. Below 1024px, hide the sidebar and open it as a drawer from a header button. Support widths down to 360px.

Below 1024px:

- Every page has the drawer button. The drawer lists the section links first, then the sidebar of the page, if the page has one. The drawer opens scrolled to the current page.
- The header shows the drawer button, the site name, and a search icon button. It hides the section links, the search hint text, and the keyboard shortcut.
- The member filter bar does not stick to the top, so it does not cover the members.
- The search dialog sits near the top of the screen, has a close button instead of the Esc hint, and hides the keyboard hints.
- A signature breaks after the class name and after the opening parenthesis before it breaks inside a name.

On touch screens (`pointer: coarse`), make controls at least 36px tall, and set inputs to 16px text so that iOS does not zoom in when an input gets focus.

Stack layers in this order, from lowest to highest:

1. Page content
2. The sticky filter bar and the sticky header
3. The mobile sidebar drawer
4. Menus and popovers
5. The search dialog
6. Tooltips

Open the search dialog with `<dialog>`, and menus, popovers, and tooltips with the `popover` attribute, so they render in the browser's top layer.

## API content

### Page structure

Every page except the home page is built from the same parts, in the same order, so that moving between pages never moves the controls the reader has learned:

1. Breadcrumbs: the pages above this one, starting with Home, as `--color-fg-muted` links separated by `›` in `--color-fg-faint`. Class pages list the Classes listing, then the inheritance path from the root class to the parent, in Geist Mono `--color-code-type`. The breadcrumbs do not repeat the current page.
2. The page icon and page name, as described in [Typography](#typography). On classes, enums, and documented data types, the title row ends with the "Open on Creator Hub" icon action, the same 24px action that members use, aligned with the member actions on the right edge of the content column.
3. A one-sentence lead, on listings and facet pages. Class and enum pages show their description in this place instead, when the API docs have one.
4. Tags, then facts, as `--color-fg-faint` labels with `--color-fg-muted` values. Classes show memory category, own members, inherited members, subclasses, siblings, and uses, then, in a second row, how many of their own members each audience can use: "Game scripts", "Plugins only", "Command bar only", and "Roblox only". Each member counts once, under the most open audience that its security allows, as the search filter `access:` does. Subclasses counts direct subclasses only. Siblings counts the classes that share the class's superclass, including the class itself; for a root class, it counts the root classes. Enums show items and uses. Data types show kind, Luau type, and uses. Facets show kind and uses.
5. The sticky filter, right below the header. Every listing and detail page has one. Its field shows the number of rows inside its right edge: "205 total" or "12 of 205".
6. Sections, each with an `h2` made of a 20px icon, the title, and the count in `--color-fg-faint`.

The Classes listing shows the whole class tree, fully expanded.

### Member filter bar

- The first row holds the name field, which takes the remaining width, and the "Filter" menu button with a `funnel-simple` icon. Both are 32px tall on `--color-raised`.
- The second row holds the 28px member kind chips and the "Collapse all" button at its right edge.
- There is no separate security select. Security levels are tags in the Filter menu, so excluding "Roblox script security" and "Roblox engine security" shows what game and plugin scripts can use.
- The "Filter" menu button shows a count badge with the number of active tag filters, counting hidden inherited members as one. It opens an anchored popover menu, 320px wide and aligned to the right edge of the button, on `--color-overlay` with `--radius-lg`, that fades and slides 4px in. It opens on the side of the button with more room and never grows past the viewport: the tag list scrolls instead. While it is open, the page keeps its height and scroll position, so filtering never moves the menu or the members around it.
- The menu starts with an "Inherited members" switch row and the "Required tags" Any/All segmented control. Below them, it lists every tag chip that a member on the page shows, grouped under Tags, Security, Thread safety, Capabilities, and Other, with the number of members that show it. Rows under Capabilities drop the repeated prefix, so "Capabilities: Basic" reads "Basic" and "Read capabilities: Basic" reads "Read: Basic"; screen readers still hear the full label. Each row has two 24px toggles: require, with a check icon, and exclude, with a prohibit icon in `--color-danger`. Turning one on turns the other off. An excluded tag's label is struck through in `--color-fg-faint`.
- A member is shown when it has any required tag (Any) or every required tag (All), and none of the excluded tags. The menu ends with a one-line explanation of this rule and a "Reset" button.
- The page builds the tag list and each member's `data-tags` keys when it is prerendered. Keys are the chip labels in lowercase kebab case, such as `read-only` or `capabilities-basic`.

### Members

- Group members into sections in this order: Properties, Methods, Events, Callbacks. Omit an empty section.
- After the members, a class page lists the members that use the class as their type, in "Used by properties", "Used by parameters", and "Used by return values" sections, as on enum and data type pages. Omit them when no member uses the class. The member filter does not filter them.
- Inside a section, group members by the class that defines them. Order the groups by inheritance, from the root class to the current class: "Inherited from Object", "Inherited from Instance", and so on, then "Defined by Part" last.
- Collapse every inherited group by default, even when the class defines no members of that kind. Expand the group of the current class. The whole group header is a button that collapses and expands the group. The filter bar has a "Collapse all" button that changes to "Expand all". Opening an anchor inside a collapsed group expands it.
- Do not write collapsed groups to the URL.
- Inside a property group, group properties by their category, such as Appearance or Behavior, under a 12px heading. The heading reads "Property category" in `--color-fg-faint`, then the category name in `--color-fg-muted`, which links to the category's Index page. Align it with the signatures.
- Sort members alphabetically within a group, reading runs of digits as numbers, so `AnimationId2` comes before `AnimationId10`. Every name list on the site sorts the same way. Put deprecated members last, with the name in `--color-fg-muted`.
- Show each member as plain text on the page, not in a box: the Luau signature on the first line, then its description, then tags, then its parameters and return value, then notes such as the preferred replacement. Enum items follow the same order, without parameters. Separate members with 16px of whitespace only. Do not add a border, a background, or a radius to a member.
- Methods, events, and callbacks show every parameter from the dump in a "Parameters" table, and methods and callbacks show their return type in a "Returns" table. Omit "Returns" when a method returns nothing. The first column holds code written as in the signature: the type-annotated parameter with its default, such as `priority: number? = 2000`, under a "Parameter" heading, or the return type under a "Type" heading. When any row has a description, add a Description column and split the table 35% code to 65% description (45% to 55% below 640px), so descriptions get the room; code wraps inside its column. When no row has a description, show the code column alone at full width. Descriptions are `--color-fg-muted` prose.
- Set these tables in `--text-sm` with a caption in `--color-fg-muted` at weight 500, column headings in `--color-fg-faint` at weight 500, a 1px `--color-line` border with `--radius-md`, and a 1px `--color-line` rule between rows. Pad cells 6px by 12px. They are the only bordered element inside a member.
- Each member has two actions, shown on hover and focus: copy a link to the member, and open the member on Creator Hub.
- Give every member of the current class an anchor in the form `#Property-Name`, `#Method-Name`, `#Event-Name`, or `#Callback-Name`. An inherited member links to the anchor on its own class page.
- Link only to other pages from a signature. The name and class prefix of a member of the current class, and the enum prefix and name of an enum item, are not links. The copy link action links to the member.

### Descriptions

- Descriptions come only from the `documentation` field of `en-us.json` in the build's `content-api-docs.zip`. Do not write, summarize, or complete descriptions by hand. Do not show its code samples or links to learn more.
- Describe classes, members, parameters, return values, enums, and enum items. Match parameter docs to the dump's parameters by name, and take the first return doc. The API docs only document the generic `Connect` arguments of events, so event parameters have no descriptions. Do not describe data types or facets on the Index pages.
- Show a description as `--text-base` prose in `--color-fg`, at most `max-w-prose` wide. Member and enum item descriptions sit behind a 1px `--color-line-strong` rule on the left, indented by 12px, like a block quote.
- The `gen` task converts the HTML into a small tree. It keeps inline code, strong and emphasized text, line breaks, lists, preformatted text, keyboard keys, superscripts, and links. It removes images and unwraps every other element. It replaces typographic quotes with plain quotes, uses LF line endings, keeps at most two line breaks in a row, and trims whitespace, line breaks, and empty elements at the edges of every block. Strong text uses `--color-fg-bright` at weight 600. Emphasized text uses `--color-fg-bright` without italics.
- Inline code that exactly names a class, data type, enum, enum item, or member links to its page on this site in its code color, like a signature. A bare name also links to a member of the documented class or an item of the documented enum; a name written as a call, such as `Pause()`, only links to a method. Code that names nothing in the API stays unlinked in `--color-fg`.
- Creator Hub reference links point to this site. Other links, such as guides, open on Creator Hub in a new tab and follow the prose link style.

### Signatures

Write each member as a line of a Luau type definition, prefixed with the class that defines it:

- Property: `Terrain.Decoration: boolean = false`
- Method: `Instance:FindFirstChild(name: string, recursive: boolean? = false): Instance?`
- Event: `BasePart.Touched: RBXScriptSignal<(otherPart: BasePart)>`
- Callback: `BindableFunction.OnInvoke: (...: any) -> ...any`
- Enum item: `Enum.ActuatorRelativeTo.Attachment0 = 0`

Follow these rules in every signature:

- Show `= value` only when the dump has a default value. When a property has no default, show nothing after its type.
- Mark optional parameters with `?`. Write arrays as `{ T }` and multiple return values as `(A, B)`. Write enum types as `Enum.Name`.
- Link every class, data type, and enum name in a signature to its page on this site. Link a primitive such as `number` to the page of the dump type that it comes from, such as `float`. Do not link a type directly to Creator Hub.
- Wrap a signature that is longer than 80 characters and has two or more parameters. Put one parameter on each line, indented by one step. Do not scroll signatures horizontally.

### Data types

- The `gen` task collects every primitive type, group type, and Roblox type that any property, parameter, or return value uses. The site lists them at `/index/types/` and builds a page for each one at `/index/types/Name/`.
- Use the labels "Primitive types", "Group types", and "Roblox types", in that order, everywhere types are grouped. These correspond to the dump categories Primitive, Group, and DataType. Use the same labels for the Kind fact on type pages.
- The API dump does not describe these types. Say so once, in plain text in the introduction of the Index page. Do not show a warning box or a warning icon for it.
- A data type page lists the members that use the type, grouped into "Used by properties", "Used by parameters", and "Used by return values". It links to Creator Hub when a page is known.

### Index

The Index section collects everything that the `gen` task derives from the dump rather than reads as a page of its own: data types, tags, property categories, security levels, capabilities, thread safety levels, and memory categories. Each derived value is a facet.

- The header link is "Index", at `/index/`. The Index page lists every facet value in sections, with the number of uses next to each one, and a filter for all of them.
- Every section has a listing at `/index/types/`, `/index/tags/`, `/index/property-categories/`, `/index/security-levels/`, `/index/capabilities/`, `/index/thread-safety/`, or `/index/memory-categories/`. Append the encoded value name and a trailing slash for its detail page. Class and enum pages follow the same listing/detail pattern at `/classes/Name/` and `/enums/Name/`.
- Keep trailing slashes on internal page URLs because each page is built as a directory containing `index.html`. File assets keep their extensions without a trailing slash. Query strings and fragments follow the page's slash.
- Index section headings and sidebar group names link to their listings. Listing pages link back to Index; detail pages link back to Index and their section. Each listing has its own URL-backed name filter.
- A facet page names the value in Geist Mono as its `h1`, says in one sentence what the page lists, and shows its facts: the kind, the number of uses, and, for a tag, the chip that members show, or, for a security level, the thread identities that can use it.
- A facet page lists its uses in sections in this order: Classes, Enums, Enum items, Properties, Methods, Events, Callbacks. Each row shows the kind icon and the name as code. When only reading or only writing a property needs the value, the row says "Read access" or "Write access".
- The `gen` task builds the facets into `api.json`. Pages only read them.
- The sidebar groups the index by facet kind, with data types split into Primitive types, Group types, and Roblox types. Only the groups that hold the current page are open. On the data type listing, all three type groups are open. Open group headers stick while their rows scroll, like the class tree.
- Property categories index only properties defined by each class, without duplicating inherited properties.
- Preserve category names exactly, including case. When names differ only in case, generation adds a numeric suffix to the colliding URL so both pages work on case-insensitive filesystems.
- The Index ends with a "Removed" section, with the `clock-counter-clockwise` icon, that lists the classes and enums removed since the oldest tracked version and not added back. Its listing is `/index/removed/`, with a Classes and an Enums section, and the sidebar has a Removed group. Removed classes and enums have no page. Each row shows the name in Geist Mono in `--color-fg-muted`, "Removed in" with the version linked to the removal, and the date.
- Only data type names use `--color-code-type`. Other facet names are identifiers, not types, and use `--color-fg`.

### Footer

- Every page ends with a footer under the content column, above a 1px `--color-line` border. The footer holds the About, Contact, Privacy policy, and GitHub links in `--text-sm` `--color-fg-muted`, and the note "This site is not affiliated with Roblox Corporation." in `--color-fg-faint`.
- On a short page, the footer sits at the bottom of the window.

### Site pages

About, Contact, and Privacy policy are site pages at `/about/`, `/contact/`, and `/privacy/`. The footer links to them.

- A site page has breadcrumbs, the page icon, the page name in Geist, and a one-sentence lead. Each section has an `h2` in `--text-lg` with no icon or count, followed by `--text-base` prose in `--color-fg` with the `doc-text` link, code, and list styles.
- A site page has no sidebar, right rail, or filter.
- Write the text once in `src/site/content/`. The page and its Markdown version both render from that text.

### Updates

The Updates section shows how the API changed from each version in `roblox-versions.json` to the next. The `dump:update` task writes one diff per version to `diffs/`, and the `gen` task builds the pages from those diffs.

- The header link is "Updates", at `/updates/`. Each update has a page at `/updates/<version>/`, such as `/updates/0.744.0.7441077/`. The sidebar lists every update, newest first, grouped by year and then by month of its publish date. Year and month rows are sticky disclosure rows in Geist, like the Index sidebar groups, and end with their number of updates in `--color-fg-faint`. Each update row shows its version and its publish date, such as "Oct 05", in `--color-fg-faint`, and its upload name, such as `version-7062aec9dbe74b14`, in a tooltip. Update rows have no icon, so the full version fits next to the date. Only the year and month that hold the current update are open; on the Updates listing, only the newest year and month are open. Every other year and month starts collapsed. The sidebar filter also matches the upload name.
- The Updates listing summarizes the 10 newest updates, newest first, and lists older updates under "Earlier updates". It does not repeat their diffs, which can run to tens of thousands of elements: each update page holds its own. Each summary splits the update into the same groups as an update page under `h3` headings named like the rail, such as "Class additions", and lists every class or enum in the group as a row: the kind icon, the name in `--color-code-type`, and, in `--text-xs` `--color-fg-faint`, its members or items added, removed, and changed after their `+`, `−`, and `~` marks, or "Details changed" or "Description changed". The row links to the class or enum on the update page. Each update is a section whose `h2` holds the `git-diff` icon, the version in Geist Mono as a link to its page, and the publish date in `--color-fg-faint`. Its facts follow: the version it is compared with, and the number of added, removed, and changed items.
- An update page names the version in Geist Mono as its `h1`, says in one sentence which two versions it compares, and shows its facts with the publish date first. It groups changes into a Classes section and an Enums section. Each section splits its classes or enums into Additions (added), Removals (removed), and Changes (everything else: added, removed, or changed members or enum items, changed details, and description changes), in that order, under `h3` headings in `--text-base` with their count in `--color-fg-faint` and a 1px `--color-line-subtle` rule below. Empty groups are omitted. The rail lists each group, such as "Class additions", with its classes or enums.
- Changes read like a unified diff. Each changed class or enum is a group under an `h4` heading: the kind icon, then the name in `--color-code-type`, linked when the class or enum still exists.
- Below the group heading, each changed member or enum item is one line of Luau, written as in [Signatures](#signatures), after a 16px gutter mark in Geist Mono:
    - `+` in `--color-success` for an added item. Marks are drawn by CSS from `data-status`, with empty alternative text, so each line carries only its screen reader label. The members and items of an added class or enum have no mark: the `+` on the class or enum heading covers them.
    - `−` in `--color-danger` for a removed item.
    - `~` in `--color-fg-faint` for a changed item whose signature did not change.
    - A changed signature shows the old line with `−`, then the new line with `+`.
- Screen readers hear "Added", "Removed", "Changed", "Before", or "After" instead of a mark. Marks are the only color on a change line outside code. Do not fill change lines with color.
- Under its line, a changed class, enum, member, or enum item lists its other changed details, indented to the signature: the label in `--color-fg-faint`, the old value struck through in `--color-fg-faint`, an arrow, and the new value in `--color-fg`. Changed tags show as tag chips under "Added tags" and "Removed tags". An added member shows its tags as on its class page.
- The heading of an added class or enum ends with "Added class" or "Added enum" in `--text-xs` `--color-fg-faint`, and lists its superclass and tags. A removed class or enum ends with "Removed class" or "Removed enum" and does not list its members or items.
- Show versions as Roblox Studio versions. Wherever the site names a version, give its upload hash too, written `version-<hash>`, such as "Roblox Studio 0.744.0.7441077 (version-7aaa6838d36c4448)".
- Description changes from the API docs sit under the line of their class, enum, member, or enum item, indented to the signature. A label in `--color-fg-faint` names what changed: "Description changed", "Description of `name` added", or "Return value description removed". The new description follows as member description prose. When there was a previous description, a "Previous description" disclosure with a `caret-right` icon shows it. A member or enum item whose only change is its description gets a `~` line. Description changes do not count as added, removed, or changed items. The update's "Descriptions" fact counts them.
- Removed members, removed enum items, and old signatures are not links.
- The filter uses `q`. It matches change names, such as `Part.Size`. A class or enum name shows its whole group. The count inside the filter field counts changes. On the Updates listing, the filter matches class and enum names, the placeholder reads "Filter the 10 newest updates", and the count names its unit, such as "617 classes and enums", so it is not mistaken for the number of updates.

### Statistics

The Statistics page, at `/stats/`, shows numbers that the `gen` task computes from the API dump and the Updates history into `stats.json`. Pages only read them.

- The header link is "Statistics". The page has breadcrumbs, the `chart-bar` icon, the page name in Geist, a one-sentence lead, and facts: the version, the oldest tracked version, and the number of updates. It has a right rail with one link per section and no sidebar or filter.
- Each section has an `h2` in `--text-lg` with no icon or count, as on site pages, and an optional `--color-fg-muted` lead.
- Counts is a table of current, removed, and total classes, services, properties, methods, events, callbacks, enums, enum items, and data types, with a total row. Services are the classes tagged Service: their row is indented under Classes, has no icon, and is left out of the total. Removed counts what an update removed since the oldest tracked version and no later update added back, so it says so above the table.
- Records are tables of Statistic, Value, and Items, grouped into Classes, Members, Enums and types, Averages, Names, History, Release rhythm, Deprecations, Comebacks, and Signature changes. Records split by kind: each record about members exists once for properties, methods, events, and callbacks, and enum items get their own, never a mixed "member" record. Each "most", "longest", or "highest" record has its opposite next to it, unless the opposite would repeat it. A value with a unit, such as days, writes the unit after it in `--color-fg-faint`, singular for 1. A removed item is code in `--color-fg-muted` that links to its removal on the update page. A shared name, such as the most common property name, is unlinked code. Every record lists every item that ties for it, as code linked like a signature. More than 8 items collapse into a disclosure that names their number, such as "80 parameters". More than 40 tied items are not listed: the Items cell names their number in `--color-fg-muted`, such as "428 classes".
- Deprecations, Comebacks, and Signature changes start with a table of counts by kind, one column per kind. A kind that a count does not apply to shows `–` in `--color-fg-faint`.
- Statistics tables use the `stat-table` utility: `--text-sm`, a 1px `--color-line` border with `--radius-md`, `--color-fg-faint` column headings at weight 500, a rule between rows, 6px by 12px cells, and right-aligned tabular numbers. A table wider than the column scrolls inside its border, never the page.
- Comebacks also lists each pair of updates that removed and restored items. Access starts with a table of the current members by audience and kind. Access, Tags, Capabilities, Descriptions, and Growth by class are tables of what each year changed and of the current API. An update in these tables is its version in Geist Mono, linked to its page, then its date in `--text-xs` `--color-fg-faint`. Tag and capability names are identifiers in `--color-fg` that link to their Index pages.
- Descriptions and Access start with a share chart: the described share of each kind, and the share of members each audience can use.
- Distributions shows, as column charts and one table, how many classes define each number of properties, methods, events, and callbacks, how many methods, events, and callbacks take each number of parameters, and how many enums have each number of items. Release days shows the updates and changes on each weekday in UTC, and the days between updates, as column charts and tables.
- Trends show the classes and enums after each update as line charts, the members each update added and removed as a bar chart, followed by one such chart each for classes, properties, methods, events, callbacks, and enum items, the changes and updates in each year and in each month of the year, across all years, as column charts, and tables of changes by year, by month, and by update. The tables have a column for each kind: classes, properties, methods, events, callbacks, enums, and enum items. A change cell writes each nonzero count after its mark, as on Updates pages: `+` added, `−` removed, and `~` changed.

### Charts

- A chart is a prerendered SVG in a `figure` with its title in `--color-fg` at weight 500 and a summary in `--color-fg-muted` on the same row. It has `role="img"` and an `aria-label` with both, and the same numbers are always in a table on the page.
- A share chart is a list of rows: the label in `--color-fg-muted`, a 6px track in `--color-line` filled with `--color-fg-muted` to the share, and the percentage in Geist Mono `--text-xs`.
- Lines are 2px `--color-fg-muted`, and so are the columns of a chart with one series. Columns label only the highest and lowest values; every value is in the tooltip and the table. Bars are thin, with a gap as wide as the bar. A bar that adds uses `--color-success` and a bar that removes uses `--color-danger`, under a legend with a swatch for each. Axes and baselines use `--color-line` or `--color-line-strong`. Axis labels are `--text-xs` `--color-fg-faint`: the top, bottom, and zero values on the left, and each year on the bottom.
- When one bar is more than twice the next tallest, the axis stops at the next tallest, the bar is cut at the axis, and its value sits at the end of the bar in `--text-xs` `--color-fg-muted`.
- Each update is a column that shows a tooltip with its version, date, and values, and fills with `--color-hover` on hover.

### History

The `gen` task reads every update, oldest first, and records each update that touched each class, enum, member, and enum item.

- A member of the current class, an enum item, and the header of a class or enum page end with a History disclosure when the Updates history knows the item. Inherited members do not have one; their own class page does. It is collapsed by default and works without JavaScript.
- Its summary reads "History" in `--color-fg-muted`, after a 14px `clock-counter-clockwise` icon and before the number of entries in `--color-fg-faint` and a `caret-right` that rotates when it opens.
- Open, it lists every update that touched the item, newest first, indented to the summary text. Each row shows the version in Geist Mono as a link, the publish date in `--text-xs` `--color-fg-faint`, and what happened in `--color-fg-muted`, as one sentence of short phrases in this order: "Added" or "Removed"; for a class or enum, its members or items, such as "2 members added, 1 changed, 1 deprecated" or "3 items removed"; "Deprecated" or "No longer deprecated"; each detail that changed, on the item itself or, for a class or enum, on any of its members or items, such as "security updated", "type updated", "default value updated", or "signature updated"; the number of tags added and removed, such as "2 tags added"; and "description updated". A change with no known detail reads "Changed". The link opens the exact change on the update page, so the history does not repeat what changed. When the item already existed in the oldest tracked version, the list ends with "Added: not recorded" and that version, in `--color-fg-faint`.
- Every change line on an update page has an anchor: `Class-Part` for a class or enum heading, and `Class-Part.Size` or `Enum-Material.Grass` for a member or enum item. The line opened by its anchor shows the same 2px `--color-fg-muted` bar as a member.
- A change counts as "Changed" only when its signature or another API detail changed. A description change alone is "Description changed".
- A Deprecated tag shows the date of the update that deprecated the item after its label, in `--color-fg-faint`, and that version and date in its tooltip. When the deprecation predates the tracked history, the tag has no date and its tooltip says so.
- Class and enum pages end with a "Removed members" or "Removed items" section, with the `clock-counter-clockwise` icon, when members or items were removed from them since the oldest tracked version and not added back. Each row is the old signature after a `−` mark, as on an update page, and below it "Removed in", the version linked to the change, and the date. The rail links to the section.
- Class and enum pages show two more facts in a second fact row: "Added", the update that last added the class or enum, and "Last changed", the last update after that which changed it or its members or items. Each links to the class or enum on that update page. When the item already existed in the oldest tracked version, "Added" reads "Not recorded", and its tooltip names that version.

### Tags

- Show tags as 20px chips: 12px text, `--radius-xs`, a 1px `--color-line` border, and no fill.
- Write tag labels in sentence case: "Deprecated", "Read only", "Not scriptable", "Not replicated", "Yields", "Plugin security".
- Color only two kinds of tags. Deprecated uses `--color-warning`. A security level that only Roblox can use, from Roblox script security upward, uses `--color-danger`. Every other tag is neutral.

### Links

- Show a link in code in its code color. Underline it on hover and focus. Do not underline it at rest.
- Show a link in prose in `--color-fg` with a 1px `--color-line-strong` underline. On hover, change the underline to `--color-fg`.
- Link every class, member, enum, and documented data type to its Creator Hub page, through the "Open on Creator Hub" action in the page title row or on the member.

### Filters and URL state

- Write every filter to the URL with `history.replaceState` once the reader pauses for 300ms, and read the URL first when the page loads. Each URL change makes Chrome re-check the speculation rules against every link, so writing on every keystroke makes typing lag. Omit a parameter when it has its default value.
- Member filters use `q` for the name, `hide` for hidden kinds and inherited members, `tags` for required tags, `not` for excluded tags, and `match=all` when every required tag must match. List filters use `q`. Sidebar filters use `nav`. The search dialog uses `search`.
- Apply member filters before the first paint with the inline script in `index.html`, so that a shared link never shows unfiltered members first.
- By default, show everything: every kind and inherited member, with no tag filters. Do not save filters in `localStorage`. A page opens with the defaults unless its URL says otherwise.

### Search

- Open search with `Ctrl+K`, `Cmd+K`, or `/`. Search opens as a dialog over a `--color-scrim` backdrop.
- Results update on every keystroke with no debounce delay that the reader can see. Group results by kind: classes, data types, enums, then members.
- Show a result as its kind icon, its name in Geist Mono, and its parent in `--color-fg-muted` for members.
- Move through results with the arrow keys. `Enter` opens the result. `Tab` and `ArrowRight` accept the completion. `Escape` closes the dialog.
- The query syntax is `Class.member`, `Class:method`, `.member`, `Enum.Name`, `Enum.Name.Item`, `"exact"`, `is:kind`, `tag:name`, `!tag:name`, `type:Name`, `uses:Name`, and `access:game|plugin|command`. `type:` matches the type of a property or the return type of a method or callback. `uses:` also matches parameter types.
- When a query names an enum, data type, or class exactly, show a "Used by" group after the other results. The group lists the members that use the type, and labels each one as a property type, a parameter, or a return value. The reader can search `uses:Name` for more.
- Search loads its index when the dialog opens, not before. Until the index is ready, the results area shows a turning `circle-notch` and "Preparing the search index…". The status appears after `--delay-loading`, so a fast load shows nothing. Typing still works, and the results appear when the index is ready.
- A query that is an upload name with or without `version-`, at least its first 6 hex digits, or the start of a version number, such as `0.745`, shows an "Updates" group first. Each row shows the `git-diff` icon, the version, the upload name in `--color-fg-faint`, and the publish date, and opens the update page.
- Removed classes, enums, members, and enum items appear in a "Removed" group after the other results when they match the query, with the same query syntax. Each row shows the kind icon, the name struck through in `--color-fg-muted`, its class or enum, and "Removed in" with the version, and opens the removal on its update page.
- Search lives only in the dialog. There is no search page. `/?search=query` opens the dialog with the query, which is also the OpenSearch template.

## States

- Hover: change the fill to `--color-hover` or the text to `--color-fg`. Do not lift, scale, or shadow an element on hover.
- Focus: show a 2px `--color-focus` outline with a 2px offset on every focusable element, using `:focus-visible`.
- Selection: mark the current page in the sidebar with a `--color-active` fill and `--color-fg-bright` text, without a bar. Mark a member opened by its anchor with a 2px `--color-fg-muted` bar on the left edge only. The bar must not shift the content.
- Disabled: use 45% opacity and `cursor: not-allowed`. Do not also change the color.
- Not found: on a missing page or an empty search, say what was not found and how to continue. For example: No results for `Humanoid.Jumpp`. Check the spelling or search for a class name.

## Motion

Motion must feel immediate. If the reader can notice that they are waiting for an animation, the animation is too slow. Follow the animation performance rules in `AGENTS.md`.

- `--duration-instant` (60ms): press feedback
- `--duration-fast` (100ms): hover color, tooltips, and icon swaps
- `--duration-exit` (90ms): every exit
- `--duration-ui` (140ms): entrances of menus, popovers, the search dialog, and the mobile sidebar
- `--ease-out` (`cubic-bezier(0.16, 1, 0.3, 1)`): entrances
- `--ease-standard` (`cubic-bezier(0.4, 0, 0.2, 1)`): exits and color changes
- `--motion-distance` (4px): entrance offset
- `--motion-scale` (0.98): entrance and press scale

What moves:

- Menus and popovers fade in and move `--motion-distance` from their anchor.
- The search dialog fades in and scales from `--motion-scale`. Its backdrop fades.
- The mobile sidebar slides in from the left edge.
- Tooltips fade. They do not move.
- Buttons scale to `--motion-scale` while pressed.
- The disclosure caret rotates 90 degrees when its group opens.
- The thumb of a switch slides to the right when it turns on.
- The copy icon crossfades to the check icon, then back after 1.5 seconds.
- The search loading spinner turns once every `--duration-spin` while the index loads. It is the only looping animation.

What does not move:

- Page navigation, scrolling, and anchor jumps. They are instant.
- Content inside an expanded group or tree node. It appears at once, with no height animation.
- Sidebar items, members, and search results. Do not stagger or fade them in.
- Anything that the reader did not trigger. There is no load animation and no idle animation.

Under `prefers-reduced-motion: reduce`, nothing moves or scales. Fades may remain at `--duration-fast` or shorter.

## Accessibility

- Give all text a contrast ratio of at least 4.5:1 against its background. The text and code tokens meet this on the surfaces listed in [Color](#color).
- Give focus rings, meaningful icons, and control boundaries a contrast ratio of at least 3:1.
- Do not rely on color alone. A deprecated member also has the Deprecated tag, and a code color always comes with a text name.
- Use landmarks: `header`, `nav` for the sidebar, `main` for the content, `aside` for the right rail, and `footer` for the footer. Add a "Skip to content" link as the first focusable element.
- Use one `h1` per page, for the page name, `h2` for member sections, and `h3` for property category headings. Do not skip levels.
- Give each landmark a unique name. Do not use `<footer>` or `<header>` inside a dialog, because browsers expose them as page landmarks.
- Give `role="listbox"` only to an element that holds options. When a combobox has no options to show, set `aria-expanded="false"`.
- Make every action available from the keyboard, and keep the focus order the same as the visual order. The one exception is the right rail, which comes before the content so that it renders early; the skip link takes keyboard readers past the sidebar and the rail.

## Writing

- Use sentence case for all interface text. Keep API names exactly as Roblox writes them, such as `GetTouchingParts` and `BasePart`.
- Use these terms consistently: class, data type, enum, enum item, member, property, method, event, callback, parameter, return value.
- Start a button label with a verb, such as "Copy link".
- Write an error as what happened, then what the reader can do. Do not apologize.
- Do not join metadata with middle dots. Put each value in its own element.
- Use digits for numbers and the ellipsis character "…" for truncation. Do not use exclamation marks.
- Write dates in UTC with a three-letter month, a two-digit day, and a four-digit year, such as "Oct 05, 2026". Where the year is already shown, as in the Updates sidebar, drop it: "Oct 05". Write a month on its own with three letters too, such as "Oct".
