import { visit } from 'unist-util-visit'

export default function quoteCitation() {
	// @ts-ignore
	return function (tree) {
		visit(tree, 'element', (node) => {
			if (node.tagName === 'blockquote') {
				// @ts-ignore
				node.children = node.children.filter((n) => n.value !== '\n')
				const last = node.children.at(-1)
				const lastText = last?.children?.at(0)?.value
				if (lastText !== undefined && (lastText.startsWith('---') || lastText.startsWith('—'))) {
					last.tagName = 'footer'
				}
			}
		})
	}
}
