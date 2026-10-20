import { matchPositions } from "../../search/score.ts"

type Segment = {
	text: string
	marked: boolean
}

function segmentsOf(name: string, word: string): Segment[] {
	const marked = new Set(matchPositions(word, name))
	const segments: Segment[] = []
	for (let index = 0; index < name.length; index++) {
		const isMarked = marked.has(index)
		const last = segments.at(-1)
		if (last !== undefined && last.marked === isMarked) {
			last.text += name.charAt(index)
		} else {
			segments.push({ text: name.charAt(index), marked: isMarked })
		}
	}
	return segments
}

export function Highlight(props: { name: string; word: string }) {
	return (
		<>
			{segmentsOf(props.name, props.word).map((segment) =>
				segment.marked ? (
					<span class="text-fg-bright">{segment.text}</span>
				) : (
					segment.text
				),
			)}
		</>
	)
}
