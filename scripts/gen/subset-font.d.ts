declare module "subset-font" {
	type SubsetOptions = {
		targetFormat?: "woff2" | "woff" | "truetype" | "sfnt"
		variationAxes?: Record<
			string,
			number | { min: number; max: number; default?: number }
		>
	}

	export default function subsetFont(
		font: Buffer,
		text: string,
		options?: SubsetOptions,
	): Promise<Buffer>
}
