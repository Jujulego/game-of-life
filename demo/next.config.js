/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  webpack: (config) => {
    config.experiments ??= {};
    config.experiments.asyncWebAssembly = true;
    config.output.webassemblyModuleFilename = 'static/wasm/[modulehash].wasm'
    config.module.rules.push({
      test: /\.wasm$/,
      type: 'webassembly/async'
    });

    return config;
  }
};

module.exports = nextConfig;
