import { aboutHref, contactHref, privacyHref } from "../api/routes.ts"
import { externalLink } from "../components/external-link.ts"
import { repositoryUrl } from "../site/repository.ts"

const linkClass = "rounded-sm motion-colors hover:text-fg"

const siteLinks = [
	{ label: "About", href: aboutHref() },
	{ label: "Contact", href: contactHref() },
	{ label: "Privacy policy", href: privacyHref() },
]

export function Footer() {
	return (
		<footer class="border-t border-line px-4 py-6 md:px-8">
			<div class="mx-auto flex max-w-content flex-col gap-3 text-sm text-fg-muted md:flex-row md:items-center md:justify-between">
				<nav aria-label="Site">
					<ul class="flex flex-wrap gap-x-4 gap-y-2">
						{siteLinks.map((link) => (
							<li>
								<a href={link.href} class={linkClass}>
									{link.label}
								</a>
							</li>
						))}
						<li>
							<a href={repositoryUrl} {...externalLink} class={linkClass}>
								GitHub
								<span class="sr-only"> (opens in a new tab)</span>
							</a>
						</li>
					</ul>
				</nav>
				<p class="text-fg-faint">
					This site is not affiliated with Roblox Corporation.
				</p>
			</div>
		</footer>
	)
}
