import { visit } from 'unist-util-visit'

export default function detailsBlock() {
	// @ts-ignore
	return function (tree) {
		visit(tree, { type: 'element', tagName: 'blockquote' }, (node) => {
			const first = node.children.at(0)
			const firstText = first?.children?.at(0)?.value
			if (firstText !== undefined && firstText.startsWith('! ')) {
				node.tagName = 'details'
				first.tagName = 'summary'
				first.children.at(0).value = firstText.slice(2)
			}
		})
	}
}
