import { purgeCSSPlugin } from '@fullhuman/postcss-purgecss';
import cssnano from 'cssnano';

export default {
  plugins: [
    purgeCSSPlugin({
      content: ['./src/orpheus/src/pages/*.rs', './generated/posts/*.html', './generated/pages/*.html'],
      fontFace: true,
      keyframes: true,
      variables: true,
      safelist: {
        greedy: [/^series-select$/, /^header-link$/]
      }
    }),
    cssnano({
      preset: 'advanced',
    }),
  ]
}