import { execSync } from 'child_process'

export default function gitHistory() {
	// @ts-ignore
	return function (_tree, file) {
		if (file.data.fm.published === undefined) {
			try {
				file.data.fm.published = execSync(
					`git log --follow --diff-filter=A --format="%as" -1 -- "${file.filename}"`,
					{
						stdio: ['pipe', 'pipe', 'ignore'],
					},
				)
					.toString()
					.trim()
			} catch (e) {
				console.error(e)
			}
		}

		if (file.data.fm.published === undefined) {
			file.data.fm.published = new Date().toISOString().split('T')[0]
		}

		const fmLines = 2 + file.contents.split('---')[1].split('\n').length
		const totalLines = file.contents.split('\n').length

		try {
			file.data.fm.revisions = execSync(
				`git log --diff-filter=M --format="%as: %s" --no-patch "-L${fmLines},${totalLines}:${file.filename}"`,
				{ stdio: ['pipe', 'pipe', 'ignore'] },
			)
				.toString()
				.trim()
			file.data.fm.updated = file.data.fm.revisions.split('\n')[0].split(':')[0]
			// eslint-disable-next-line no-empty
		} catch {}
	}
}
