const separators = [
  `</span>`,
  `<span class="spoiler" onclick="this.classList.toggle('revealed')">`,
]

export async function onRequest(_, next) {
  const response = await next()
  const html = await response.text()
  const parts = html.split('||')

  return new Response(
    parts.reduce((acc, cur, index) => {
      if (index === 0) {
        return cur
      }

      return acc + separators[index % 2] + cur
    }, ''),
    {
      status: response.status,
      headers: response.headers,
    },
  )
}
