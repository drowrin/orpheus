import antfu from '@antfu/eslint-config'

export default antfu(
  {
    formatters: {
      astro: true,
      css: true,
      html: true,
      markdown: false,
    },
    astro: true,
    svelte: true,
    typescript: {
      tsconfigPath: 'tsconfig.json',
    },
    markdown: false,
  },
)
