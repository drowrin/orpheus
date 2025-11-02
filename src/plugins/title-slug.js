export default function titleSlug() {
	// @ts-ignore
	return function (_tree, file) {
		file.data.fm.slug = file.filename.split('/').at(-1).split('.').at(0)
	}
}
