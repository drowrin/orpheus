const separators = [
  `<span class="spoiler" onclick="this.classList.toggle('revealed')">`,
  `</span>`,
]

export async function onRequest(context, next) {
  const response = await next()

  if (context.url.search.includes('_image')) {
    return response
  }

  if (!context.url.pathname.startsWith('/posts')) {
    return response
  }

  const html = await response.text()
  const parts = html.split('||')

  return new Response(
    parts.reduce((acc, cur, index) => {
      acc.push(cur)
      acc.push(separators[index % 2])

      return acc
    }, []).join(''),
    {
      status: response.status,
      headers: response.headers,
    },
  )
}
