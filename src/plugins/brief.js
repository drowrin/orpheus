import { toHtml } from 'hast-util-to-html'
import { toText } from 'hast-util-to-text'
import { EXIT, visit } from 'unist-util-visit'

export default function removeNewlines() {
  // @ts-ignore
  return function (tree, file) {
    if (file.data.astro.frontmatter.brief) {
      file.data.astro.frontmatter.brief = {
        html: `<p>${file.data.astro.frontmatter.brief}</p>`,
        text: file.data.astro.frontmatter.brief,
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
      file.data.astro.frontmatter.brief = {
        html: toHtml(firstP),
        text: toText(firstP),
      }
    }
  }
}
