import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";

const is_watch = !!process.env.ROLLUP_WATCH;

export default {
    input: {
        index: "./Cargo.toml"
    },
    output: {
        dir: "dist/js",
        format: "iife",
        sourcemap: true,
    },
    watch: ["dist/js/assets/bundle.css"],
    plugins: [
        rust({
            serverPath: "js/",
            debug: false,
        }),

        is_watch && serve({
            contentBase: "dist",
            open: true,
            port: 8080
        }),

        is_watch && livereload("dist"),
    ],
};
