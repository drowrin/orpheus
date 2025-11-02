import getReadingTime from 'reading-time'

export default function readingTime() {
	// @ts-ignore
	return function (_tree, file) {
		file.data.fm.readingTime = getReadingTime(file.contents)
	}
}
