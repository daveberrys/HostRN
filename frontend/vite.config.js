import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte()],
  envPrefix: ['VITE_', 'PUBLIC_'],
  server: { port: 4678 },
  preview: { port: 4678 }
})
