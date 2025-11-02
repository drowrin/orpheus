import { visit, EXIT } from 'unist-util-visit'
import { toText } from 'hast-util-to-text'
import { toHtml } from 'hast-util-to-html'

export default function removeNewlines() {
	// @ts-ignore
	return function (tree, file) {
		if (file.data.fm.brief) {
			file.data.fm.brief = {
				html: `<p>${file.data.fm.brief}</p>`,
				text: file.data.fm.brief,
			}
			return
		}

		let firstP

		visit(tree, 'element', (node) => {
			if (node.tagName === 'p') {
				firstP = node
				return EXIT
			}
		})

		if (firstP) {
			file.data.fm.brief = {
				html: toHtml(firstP),
				text: toText(firstP),
			}
		}
	}
}
