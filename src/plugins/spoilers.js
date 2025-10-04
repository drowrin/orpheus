import { visit } from 'unist-util-visit'

export default function spoilers() {
  return function (tree) {
    visit(tree, 'text', (node, index, parent) => {
      if (node.value.includes('||')) {
        const before = parent.children.slice(0, index)
        const after = parent.children.slice(index + 1)

        const parts = node.value.split('||')

        const between = []
        for (const [i, v] of parts.entries()) {
          const n = { type: 'text', value: v }
          if (i % 2 === 0) {
            between.push(n)
          }
          else {
            between.push({
              type: 'element',
              tagName: 'span',
              properties: {
                class: 'spoiler',
                onclick: 'this.classList.toggle(\'revealed\')',
              },
              children: [n],
            })
          }
        }

        parent.children = [...before, ...between, ...after]
      }
    })
  }
}
