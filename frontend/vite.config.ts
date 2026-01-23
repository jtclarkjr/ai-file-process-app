import { sveltekit } from '@sveltejs/kit/vite';
import { createLogger, defineConfig } from 'vite';

// Custom logger to suppress known Vite 8 deprecation warnings from SvelteKit
const logger = createLogger();
const originalWarn = logger.warn.bind(logger);
logger.warn = (msg, options) => {
  // Suppress esbuildOptions deprecation warning until SvelteKit fully supports Vite 8
  if (msg.includes('optimizeDeps.esbuildOptions')) return;
  originalWarn(msg, options);
};

export default defineConfig({
  plugins: [sveltekit()],
  customLogger: logger,
  clearScreen: false,
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
});
