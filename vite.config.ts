import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  base: '/gomoku-alpha/', // GitHub Pagesのリポジトリ名を指定
})
