import { negotiate, type Assets } from "../src/edge/negotiate.ts"

type PagesContext = {
	request: Request
	next: () => Promise<Response>
	env: { ASSETS: Assets }
}

export function onRequest(context: PagesContext): Promise<Response> {
	return negotiate({
		request: context.request,
		next: () => context.next(),
		assets: context.env.ASSETS,
	})
}
