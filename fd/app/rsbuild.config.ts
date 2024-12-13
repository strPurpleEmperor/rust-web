import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';

export default defineConfig({
  plugins: [pluginReact()],
  source: {
    entry: {
      index: './src/pages/index/index.tsx',
      user: './src/pages/user/index.tsx',
    }
  }
});
