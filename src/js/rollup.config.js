import { nodeResolve } from '@rollup/plugin-node-resolve';

export default {
    input: 'src/sender.js',
    inlineDynamicImports: true,
    output: {
        file: 'dist/bundle_glue.js',
        format: 'esm',
        exports: 'named'
    },
    plugins: [nodeResolve()]
}