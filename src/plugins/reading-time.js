import getReadingTime from 'reading-time'

export default function readingTime() {
  // @ts-ignore
  return function (_tree, file) {
    file.data.astro.frontmatter.readingTime = getReadingTime(file.value)
  }
}
