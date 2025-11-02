import slugify from 'slugify'

export default function seriesSlug() {
	// @ts-ignore
	return function (_tree, file) {
		if (file.data.fm.series) {
			file.data.fm.series = {
				name: file.data.fm.series,
				slug: slugify(file.data.fm.series, { lower: true }),
			}
		}
	}
}
