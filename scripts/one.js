const fs = require('node:fs');
const path = require('node:path');

const ROOT = path.resolve('mujs');
const OUTPUT = path.resolve('mujs-one.c');

const TPL = `
#include "jsi.h"
#include "jsarray.c"
#include "jsboolean.c"
#include "jsbuiltin.c"
#include "jscompile.c"
#include "jsdate.c"
#include "jsdtoa.c"
#include "jserror.c"
#include "jsfunction.c"
#include "jsgc.c"
#include "jsintern.c"
#include "jslex.c"
#include "jsmath.c"
#include "jsnumber.c"
#include "jsobject.c"
#include "json.c"
#include "jsparse.c"
#include "jsproperty.c"
#include "jsregexp.c"
#include "jsrepr.c"
#include "jsrun.c"
#include "jsstate.c"
#include "jsstring.c"
#include "jsvalue.c"
#include "regexp.c"
#include "utf.c"
`.trim();

const INCLUDE_RE = /^\s*#\s*include\s*"([^"]+)"\s*(?:\/\/.*)?$/;
const SOURCE_EXTENSIONS = new Set(['.c', '.h']);

function parseInclude(line) {
  const match = line.match(INCLUDE_RE);
  if (!match) return null;

  const filename = match[1];
  if (!SOURCE_EXTENSIONS.has(path.extname(filename))) return null;

  return filename;
}

function relativePath(filename) {
  return path.relative(process.cwd(), filename) || filename;
}

function resolveInclude(dir, filename, required = false) {
  const resolved = path.resolve(dir, filename);

  if (!fs.existsSync(resolved)) {
    if (!required && path.extname(filename) === '.h') {
      return null;
    }

    throw new Error(`Include file not found: ${filename}\n  from: ${dir}`);
  }

  if (!fs.statSync(resolved).isFile()) {
    throw new Error(`Include is not a file: ${resolved}`);
  }

  return resolved;
}

function inlineFile(filename, state) {
  filename = path.resolve(filename);

  // Detect cycles before checking duplicates.
  if (state.active.has(filename)) {
    const chain = [...state.stack, filename]
      .map(relativePath)
      .join('\n  -> ');

    throw new Error(`Circular include detected:\n  ${chain}`);
  }

  // Each source/header file is expanded at most once.
  if (state.included.has(filename)) {
    return `/* Already included: ${relativePath(filename)} */`;
  }

  state.included.add(filename);
  state.active.add(filename);
  state.stack.push(filename);

  try {
    const source = fs.readFileSync(filename, 'utf8');
    const dir = path.dirname(filename);
    const output = [];

    for (const line of source.split(/\r?\n/)) {
      const include = parseInclude(line);

      if (!include) {
        output.push(line);
        continue;
      }

      const resolved = resolveInclude(dir, include);

      // Preserve unresolved quoted headers, which may be external dependencies.
      if (!resolved) {
        output.push(line);
        continue;
      }

      output.push(`\n/* BEGIN: ${relativePath(resolved)} */`);
      output.push(inlineFile(resolved, state));
      output.push(`/* END: ${relativePath(resolved)} */\n`);
    }

    return output.join('\n');
  } finally {
    state.stack.pop();
    state.active.delete(filename);
  }
}

function buildSingleFile(rootDir, template) {
  const state = {
    included: new Set(),
    active: new Set(),
    stack: [],
  };

  const output = [];

  for (const line of template.split(/\r?\n/)) {
    const include = parseInclude(line);

    if (!include) {
      output.push(line);
      continue;
    }

    // Template dependencies are required; fail early if one is missing.
    const filename = resolveInclude(rootDir, include, true);

    output.push(`\n/* BEGIN: ${relativePath(filename)} */`);
    output.push(inlineFile(filename, state));
    output.push(`/* END: ${relativePath(filename)} */\n`);
  }

  return output.join('\n') + '\n';
}

function main() {
  const code = buildSingleFile(ROOT, TPL);

  fs.writeFileSync(OUTPUT, code, 'utf8');

  console.log(
    `Generated ${relativePath(OUTPUT)} (${Buffer.byteLength(code)} bytes)`
  );
}

try {
  main();
} catch (error) {
  console.error(error.message);
  process.exitCode = 1;
}