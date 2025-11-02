import { visit } from 'unist-util-visit'
import { toText } from 'hast-util-to-text'
import slugify from 'slugify'

export default function headings() {
	// @ts-ignore
	return function (tree, file) {
		file.data.fm.headings = []

		visit(tree, 'element', (node) => {
			if (['h1', 'h2', 'h3', 'h4', 'h5', 'h6'].includes(node.tagName)) {
				const depth = parseInt(node.tagName[1])
				const text = toText(node)
				const slug = slugify(text, { lower: true, strict: true })
				node.properties.id = slug
				file.data.fm.headings.push({
					depth,
					slug,
					text,
				})
			}
		})
	}
}
