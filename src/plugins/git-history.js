import { execSync } from 'node:child_process'
import { readFileSync } from 'node:fs'

export default function gitHistory() {
  // @ts-ignore
  return function (_tree, file) {
    if (file.data.astro.frontmatter.published === undefined) {
      try {
        file.data.astro.frontmatter.published = execSync(
          `git log --follow --diff-filter=A --format="%as" -1 -- "${file.history[0]}"`,
          {
            stdio: ['pipe', 'pipe', 'ignore'],
          },
        )
          .toString()
          .trim()
      }
      catch (e) {
        console.error(e)
      }
    }

    if (!file.data.astro.frontmatter.published) {
      file.data.astro.frontmatter.published = new Date().toISOString().split('T')[0]
    }

    const contents = readFileSync(file.history[0], 'utf-8')

    const fmLines = 2 + contents.split('---')[1].split('\n').length
    const totalLines = contents.split('\n').length

    try {
      file.data.astro.frontmatter.revisions = execSync(
        `git log --diff-filter=M --format="%as: %s" --no-patch "-L${fmLines},${totalLines}:${file.history[0]}"`,
        { stdio: ['pipe', 'pipe', 'ignore'] },
      )
        .toString()
        .trim()
      file.data.astro.frontmatter.updated = file.data.astro.frontmatter.revisions.split('\n')[0].split(':')[0]
    }
    catch {}
  }
}
