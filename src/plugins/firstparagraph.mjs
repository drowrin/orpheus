import { toString } from 'mdast-util-to-string'

export default function () {
  return function (tree, file) {
    if (file.history[0].includes('/posts/')) {
      const firstparagraph = tree.children.filter(n => n.type === 'paragraph').at(0)
      file.data.astro.frontmatter.brief = file.data.astro.frontmatter.brief
        || (firstparagraph ? toString(firstparagraph).replaceAll('\n', ' ') : '')
    }
  }
}
