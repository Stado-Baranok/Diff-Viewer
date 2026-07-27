// diff_viewer.js
#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const { program } = require('commander');
const Diff = require('diff');
const chalk = require('chalk');

program
    .argument('<file1>')
    .argument('<file2>')
    .option('--context <n>', 'Context lines', '3')
    .option('--ignore-space', 'Ignore trailing spaces', false)
    .option('--unified', 'Unified format', true)
    .option('--color', 'Force color output', false)
    .option('--output <file>', 'Output file')
    .parse(process.argv);

const opts = program.opts();
const file1 = program.args[0];
const file2 = program.args[1];
const color = opts.color || process.stdout.isTTY;

function readFile(file) {
    try {
        return fs.readFileSync(file, 'utf8').split('\n');
    } catch (err) {
        console.error(`Error reading ${file}: ${err.message}`);
        process.exit(1);
    }
}

function colorize(text, colorName) {
    if (!color) return text;
    return chalk[colorName](text);
}

let lines1 = readFile(file1);
let lines2 = readFile(file2);

if (opts.ignoreSpace) {
    lines1 = lines1.map(l => l.replace(/\s+$/, ''));
    lines2 = lines2.map(l => l.replace(/\s+$/, ''));
}

// Используем diffLines для построчного сравнения
const diff = Diff.diffLines(lines1.join('\n'), lines2.join('\n'));

let output = [];
output.push(colorize(`--- ${file1}`, 'blue'));
output.push(colorize(`+++ ${file2}`, 'blue'));

let oldLine = 0, newLine = 0;
for (const part of diff) {
    if (part.added) {
        const lines = part.value.split('\n').filter(l => l !== '');
        for (const line of lines) {
            output.push(colorize(`+${line}`, 'green'));
        }
        newLine += lines.length;
    } else if (part.removed) {
        const lines = part.value.split('\n').filter(l => l !== '');
        for (const line of lines) {
            output.push(colorize(`-${line}`, 'red'));
        }
        oldLine += lines.length;
    } else {
        const lines = part.value.split('\n').filter(l => l !== '');
        for (const line of lines) {
            output.push(` ${line}`);
        }
        oldLine += lines.length;
        newLine += lines.length;
    }
}

const result = output.join('\n');
if (opts.output) {
    fs.writeFileSync(opts.output, result);
    console.log(`Diff saved to ${opts.output}`);
} else {
    console.log(result);
}
