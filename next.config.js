/** @type {import('next').NextConfig} */
module.exports = {
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
