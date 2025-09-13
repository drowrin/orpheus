import antfu from '@antfu/eslint-config'

export default antfu(
  {
    formatters: true,
    astro: true,
    typescript: {
      tsconfigPath: 'tsconfig.json',
    },
    markdown: false,
  },
)
