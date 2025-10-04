import { visit } from 'unist-util-visit'

export default function removeNewlines() {
  return function (tree) {
    visit(tree, 'text', (node, index, parent) => {
      if (node.value === '\n') {
        const before = parent.children.slice(0, index)
        const after = parent.children.slice(index + 1)
        parent.children = [...before, ...after]
      }
    })
  }
}
