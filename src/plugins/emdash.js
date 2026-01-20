import { visit } from 'unist-util-visit'

export default function emdash() {
  return function (tree) {
    visit(tree, 'text', (node) => {
      node.value = node.value.replaceAll('---', '—')
    })
  }
}
