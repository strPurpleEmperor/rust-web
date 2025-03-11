import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import { pluginLess } from '@rsbuild/plugin-less';

export default defineConfig({
  plugins: [pluginReact(), pluginLess()],
  source: {
    entry: {
      index: './src/pages/index/index.tsx',
      user: './src/pages/user/index.tsx',
    },
  },
  output: {
    filename: {
      js: '[name].[contenthash].js',
    },
    cssModules: {
      auto: true,
      localIdentName: '[local]_[hash:8]',
    },
  },
  server: {
    port: 8051,
    proxy: [
      {
        context: ['/api'],
        target: 'http://127.0.0.1:8050',
        changeOrigin: true,
      },
    ],
  },
});
