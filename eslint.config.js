import neostandard from 'neostandard'

export default neostandard({
  ignores: [
    'src-tauri/*',
    'vite.config.ts',
    'vite-env.d.ts',
    'dist/*',
    'src/bindings.ts',
  ],
  ts: true,
  noStyle: true,
})
