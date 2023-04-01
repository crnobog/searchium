import * as esbuild from "esbuild";
import { copy } from "esbuild-plugin-copy";

let prod = process.argv.indexOf('--prod') >= 0;

const baseConfig: esbuild.BuildOptions = {
    bundle: true,
    minify: prod,
    sourcemap: !prod,
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
    entryPoints: ["./src/webview/controls.ts", "./src/webview/details.ts"],
    external: ["acquireVsCodeApi"],
    outdir: "./out/webview",
    plugins: [
        copy({
            resolveFrom: "cwd",
            assets: {
                from: ["./src/webview/*.css", "./src/webview/*.ttf"],
                to: ["./out/webview"],
            },
        }),
        copy({
            resolveFrom: "cwd",
            assets: {
                from: ["./node_modules/@vscode/codicons/dist/*.css"],
                to: ["./out/webview"],
            },
        }),
    ],
};

(async () => {
    const args = process.argv.slice(2);
    try {
        if (args.includes("--watch")) {
            // Build and watch extension and webview code
            console.log("[watch] build started");
            let ctx1 = await esbuild.context({
                ...extensionConfig,
            });
            let ctx2 = await esbuild.context({
                ...webviewConfig,
            });
            await Promise.all(
                [ctx1.watch(),
                ctx2.watch()]
            );
            console.log("[watch] build finished");
        } else {
            // Build extension and webview code
            await esbuild.build(extensionConfig);
            await esbuild.build(webviewConfig);
            console.log("build complete");
        }
    } catch (err: any) {
        console.error(err.toString());
        console.error(err.stack);
        process.stderr.write(err.stderr);
        process.exit(1);
    }
})();