import { toString } from 'mdast-util-to-string'
import getReadingTime from 'reading-time'

export default function () {
  return function (tree, file) {
    if (file.history[0].includes('/posts/')) {
      const textOnPage = toString(tree)
      const readingTime = getReadingTime(textOnPage)

      file.data.astro.frontmatter.wordCount = readingTime.words
      file.data.astro.frontmatter.readingTime = readingTime.text
    }
  }
}
