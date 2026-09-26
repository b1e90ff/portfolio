import { readFileSync } from 'node:fs';
import { registerHooks } from 'node:module';
import { pathToFileURL } from 'node:url';

// Mirrors the page's import map so the browser modules load unchanged under Node.
const vendor = readFileSync('src/view/layout.rs', 'utf8').match(/THREE_VENDOR: &str = "\/(vendor\/three-[\d.]+)"/);
if (!vendor) throw new Error('THREE_VENDOR not found in src/view/layout.rs');
const base = pathToFileURL(`public/${vendor[1]}/`).href;

registerHooks({
    resolve(specifier, context, next) {
        if (specifier === 'three') return { url: `${base}three.module.js`, shortCircuit: true };
        if (specifier.startsWith('three/addons/')) return { url: base + specifier.slice('three/'.length), shortCircuit: true };
        return next(specifier, context);
    },
});
