module.exports = {
  entry: './src/Lib.ts',
  output: {
    filename: '../static/main.js',
    libraryTarget: 'var',
    library: 'Luten'
  },
  resolve: {
    // Add `.ts` and `.tsx` as a resolvable extension.
    extensions: ['.ts', '.tsx', '.js']
  },
  module: {
    loaders: [
      {
        // All files with a `.ts` or `.tsx` extension will be handled by `ts-loader`
        test: /\.tsx?$/,
        loader: 'ts-loader'
      }
    ]
  }
}
