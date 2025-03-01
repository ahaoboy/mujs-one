const fs = require('fs');
const path = require('path');
const files = new Set();
function isInclude(referencerDir, line) {
  line = line.trim();
  if (!line.startsWith('#include')) {
    return null;
  }
  const match = line.match(/"(.+?)"/);
  if (match) {
    const relativeFile = match[1];
    if ((relativeFile.endsWith('.c') || relativeFile.endsWith('.h'))) {
      const p = path.join(referencerDir, relativeFile);
      const txt = fs.readFileSync(p, 'utf8')
      return txt;
    }
  }
  return null;
}

function dfs(dir, txt) {
  let currentText = '';
  try {
    // const dir = path.dirname(starter);
    // const lines = fs.readFileSync(starter, 'utf8').split('\n');
    const lines = txt.split('\n');
    for (const line of lines) {
      const include = isInclude(dir, line);
      if (include) {
        if (files.has(include)) {
          continue
        }
        files.add(include)
        currentText += dfs(dir, include);
      } else {
        currentText += line + '\n';
      }
    }
  } catch (error) {
    if (error.code === 'ENOENT') {
      throw new Error(`not found: ${starter}`);
    }
    throw error;
  }
  return '\n' + currentText + '\n';
}

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
`.trim()

function getCode(dir, tpl) {
  return dfs(dir, tpl)
}

function main() {
  const code = getCode('mujs', TPL);
  fs.writeFileSync('mujs-one.c', code);
}

main();