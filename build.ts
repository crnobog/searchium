import * as esbuild from "esbuild";
import { copy } from "esbuild-plugin-copy";
import * as child_process from "child_process";
import * as fs from "fs/promises";

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

const testConfig: esbuild.BuildOptions = { 
    bundle : false,
    minify : false,
    sourcemap : true,
    format: "cjs",
    entryPoints : ["./src/test/**/*.test.ts"],
    outdir : "out",
    outbase : "src",
};

const webviewConfig: esbuild.BuildOptions = {
    ...baseConfig,
    target: "es2020",
    format: "esm",
    entryPoints: ["./src/views/controls.ts", "./src/views/details.ts"],
    external: ["acquireVsCodeApi"],
    outdir: "./out/webview",
    plugins: [
        copy({
            resolveFrom: "cwd",
            assets: {
                from: ["./src/views/*.css", "./src/views/*.ttf"],
                to: ["./out/webview"],
            },
        }),
        copy({
            resolveFrom: "cwd",
            assets: {
                from: ["./node_modules/@vscode/codicons/dist/*.css", "./node_modules/@vscode/codicons/dist/*.ttf"],
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
            let ctx1 = esbuild.context({
                ...extensionConfig,
            });
            let ctx2 = esbuild.context({
                ...webviewConfig,
            });
            let ctx3 = esbuild.context({
                ...testConfig,
            });
            let watchers = await Promise.all([ctx1, ctx2, ctx3]);
            await Promise.all(watchers.map(x => x.watch()));
            console.log("[watch] build finished");
        } else {
            // Build extension and webview code
            let promises = new Array();
            promises.push(esbuild.build(extensionConfig));
            if (!prod) {
                promises.push(esbuild.build(testConfig));
            }
            promises.push(esbuild.build(webviewConfig));
            await Promise.all(promises);
            console.log("build complete");
        }
        if (prod) { 
            await fs.copyFile("./target/release/searchium-server.exe", "./bin/searchium-server.exe");
        }
    } catch (err: any) {
        console.error(err.toString());
        console.error(err.stack);
        process.stderr.write(err.stderr);
        process.exit(1);
    }
})();