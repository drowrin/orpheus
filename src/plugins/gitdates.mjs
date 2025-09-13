import { execSync } from 'node:child_process'
import fs from 'node:fs'

export default function () {
  return function (_, file) {
    if (file.history[0].includes('/posts/')) {
      const filepath = file.history[0]

      const published = file.data.astro.frontmatter.published
        || execSync(
          `git log --follow --diff-filter=A --format="%as" -1 -- "${filepath}"`,
        ).toString().trim()

      const content = fs.readFileSync(filepath, 'utf8')

      const fmLines = 2 + content.split('---')[1].split('\n').length
      const totalLines = content.split('\n').length

      const revisions = execSync(
        `git log --diff-filter=M --format="%as: %s" --no-patch "-L${fmLines},${totalLines}:${filepath}"`,
      ).toString().trim()

      file.data.astro.frontmatter.published = published
      file.data.astro.frontmatter.revisions = revisions
      file.data.astro.frontmatter.updated = revisions.split('\n')[0].split(':')[0]
    }
  }
}
