import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve-proxy";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";

const is_watch = !!process.env.ROLLUP_WATCH;

export default {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        dir: "dist/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "js/",
            debug: true
        }),
        is_watch && serve({
            contentBase: "dist",
            open: true,
            proxy: {
                api: "http://127.0.0.1:3000"
            },
        }),
        is_watch && livereload("dist"),
        !is_watch && terser(),
    ],
};