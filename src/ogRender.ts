import type { RenderFunctionInput } from 'astro-opengraph-images'
import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'

type Children = string | ElementLike | ElementLike[]
type Style = Record<string, string>
type Props = Record<string, string>

interface ElementLike {
  type: string
  props: {
    children: Children
    style: Style
  }
}

function n(type: string, style?: Style, children?: Children, props?: Props): ElementLike {
  const s = style ?? {}
  if (Array.isArray(children)) {
    s.display = 'flex'
    if (s.flexDirection === undefined) {
      s.flexDirection = 'column'
    }
  }
  return {
    type,
    props: {
      children: children ?? [],
      style: s,
      ...props,
    },
  }
}

async function renderPost(input: RenderFunctionInput) {
  const tagline = input.document.querySelector('p')?.textContent
  const tags = input.document.querySelectorAll('small li a')
  const details = input.document.querySelectorAll('small span')

  return n(
    'div',
    {
      height: '100%',
      width: '100%',
      padding: '20px',
      backgroundColor: '#1e1e2e',
      fontFamily: 'Atkinson Hyperlegible Next',
    },
    [n(
      'div',
      {
        justifyContent: 'center',
        margin: 'auto',
      },
      [
        n(
          'h1',
          { color: '#b4befe', fontSize: '80', margin: '0' },
          input.title,
        ),
        n(
          'p',
          { color: '#6c7086', fontSize: '56', margin: '0' },
          tagline,
        ),
        n(
          'div',
          {
            fontSize: '48',
            margin: '0',
            flexDirection: 'row',
            flexWrap: 'wrap',
            gap: '20px',
          },
          tags.values().map(e => n(
            'span',
            { color: '#89b4fa', textDecoration: 'underline' },
            e.textContent,
          )).toArray(),
        ),
        n(
          'p',
          { color: '#6c7086', fontSize: '48', margin: '0' },
          details.values().map(e => e.textContent).toArray().join(' - '),
        ),
      ],
    )],
  )
}

const iconPath = path.join(process.cwd(), 'public', 'favicon.svg')
const iconBase64 = `data:image/svg+xml;base64,${fs.readFileSync(iconPath).toString('base64')}`

export async function ogRender(input: RenderFunctionInput) {
  if (input.pathname.startsWith('/posts/') && !input.pathname.endsWith('/')) {
    return renderPost(input)
  }

  return n(
    'div',
    {
      height: '100%',
      width: '100%',
      padding: '20px',
      backgroundColor: '#1e1e2e',
      fontFamily: 'Atkinson Hyperlegible Next',
    },
    [n(
      'img',
      {
        height: '100%',
        color: '#b4befe',
        margin: 'auto',
      },
      [],
      {
        src: iconBase64,
      },
    )],
  )
}
