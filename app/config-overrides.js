const webpack = require('webpack');

module.exports = function override(config, env) {
  let loaders = config.resolve;
  loaders.fallback = {
    stream: require.resolve('stream-browserify'),
    buffer: require.resolve('buffer'),
    os: require.resolve('os-browserify/browser'),
  };
  config.plugins.push(
    new webpack.ProvidePlugin({
      Buffer: ['buffer', 'Buffer'],
    }),
  );

  return config;
};
