import slugify from 'slugify'

export default function seriesSlug() {
  // @ts-ignore
  return function (_tree, file) {
    if (file.data.astro.frontmatter.series) {
      file.data.astro.frontmatter.series = {
        name: file.data.astro.frontmatter.series,
        slug: slugify(file.data.astro.frontmatter.series, { lower: true }),
      }
    }
  }
}
