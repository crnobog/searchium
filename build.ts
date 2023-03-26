import * as esbuild from "esbuild";
import { copy } from "esbuild-plugin-copy";

const baseConfig: esbuild.BuildOptions = {
    bundle: true,
    minify: process.env.NODE_ENV === 'PRODUCTION',
    sourcemap: true
};

const extensionConfig: esbuild.BuildOptions = {
    ...baseConfig,
    format: "cjs",
    entryPoints: ["./src/extension.ts"],
    outfile: "./out/extension.js",
    external: ["vscode"],
    platform: 'node',
};


const webviewConfig: esbuild.BuildOptions = {
    ...baseConfig,
    target: "es2020",
    format: "esm",
    entryPoints: ["./src/webview/main.ts"],
    external: ["acquireVsCodeApi"],
    outfile: "./out/webview.js",
    plugins: [
        // Copy webview css and ttf files to `out` directory unaltered
        copy({
            resolveFrom: "cwd",
            assets: {
                from: ["./src/webview/*.css", "./src/webview/*.ttf"],
                to: ["./out"],
            },
        }),
    ],
};

const watchConfig = {
    watch: {
        onRebuild(error: any, result: any) {
            console.log("[watch] build started");
            if (error) {
                error.errors.forEach((error: any) =>
                    console.error(
                        `> ${error.location.file}:${error.location.line}:${error.location.column}: error: ${error.text}`
                    )
                );
            } else {
                console.log("[watch] build finished");
            }
        },
    },
};

(async () => {
    const args = process.argv.slice(2);
    try {
        if (args.includes("--watch")) {
            // Build and watch extension and webview code
            console.log("[watch] build started");
            await esbuild.build({
                ...extensionConfig,
                ...watchConfig,
            });
            await esbuild.build({
                ...webviewConfig,
                ...watchConfig,
            });
            console.log("[watch] build finished");
        } else {
            // Build extension and webview code
            await esbuild.build(extensionConfig);
            await esbuild.build(webviewConfig);
            console.log("build complete");
        }
    } catch (err: any) {
        process.stderr.write(err.stderr);
        process.exit(1);
    }
})();