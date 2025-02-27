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
    if (   (relativeFile.endsWith('.c') || relativeFile.endsWith('.h'))) {
      return path.join(referencerDir, relativeFile);
    }
  }
  return null;
}

function generateAmalgamatedCode(starter) {
  let currentText = '';
  try {
    const currentDir = path.dirname(starter);
    const lines = fs.readFileSync(starter, 'utf8').split('\n');
    for (const line of lines) {
      const fileToInclude = isInclude(currentDir, line);
      if (fileToInclude) {
        if(files.has(fileToInclude)){
          continue
        }
        files.add(fileToInclude)
        currentText += generateAmalgamatedCode(fileToInclude);
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

function main() {
  const amalgamatedCode = generateAmalgamatedCode('mujs/one.c');
  fs.writeFileSync('mujs_all.c', amalgamatedCode);
}

main();