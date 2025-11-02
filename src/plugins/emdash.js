import { visit } from 'unist-util-visit'

export default function emdash() {
	// @ts-ignore
	return function (tree) {
		visit(tree, 'text', (node) => {
			node.value = node.value.replace('---', '—')
		})
	}
}
