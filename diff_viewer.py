# diff_viewer.py
import sys
import argparse
import difflib
import os
from datetime import datetime

# ANSI color codes
COLORS = {
    'red': '\033[91m',
    'green': '\033[92m',
    'blue': '\033[94m',
    'reset': '\033[0m'
}

def colorize(text, color, enabled):
    if not enabled:
        return text
    return f"{COLORS.get(color, '')}{text}{COLORS['reset']}"

def read_file(path):
    try:
        with open(path, 'r', encoding='utf-8') as f:
            return f.read().splitlines()
    except Exception as e:
        print(f"Error reading {path}: {e}", file=sys.stderr)
        sys.exit(1)

def main():
    parser = argparse.ArgumentParser(description="Diff Viewer")
    parser.add_argument('file1', help='First file')
    parser.add_argument('file2', help='Second file')
    parser.add_argument('--context', type=int, default=3, help='Context lines')
    parser.add_argument('--ignore-space', action='store_true', help='Ignore trailing spaces')
    parser.add_argument('--unified', action='store_true', default=True, help='Unified format')
    parser.add_argument('--color', action='store_true', help='Force color output')
    parser.add_argument('--output', help='Output file')
    args = parser.parse_args()

    color = args.color or sys.stdout.isatty()
    lines1 = read_file(args.file1)
    lines2 = read_file(args.file2)

    # Подготовка
    if args.ignore_space:
        lines1 = [line.rstrip() for line in lines1]
        lines2 = [line.rstrip() for line in lines2]

    # Генерируем diff
    diff = difflib.unified_diff(
        lines1, lines2,
        fromfile=args.file1, tofile=args.file2,
        lineterm='',
        n=args.context
    )
    diff_lines = list(diff)

    if not diff_lines:
        print("No differences found.")
        return

    # Форматируем вывод с цветами
    output_lines = []
    for line in diff_lines:
        if line.startswith('---') or line.startswith('+++') or line.startswith('@@'):
            output_lines.append(colorize(line, 'blue', color))
        elif line.startswith('-'):
            output_lines.append(colorize(line, 'red', color))
        elif line.startswith('+'):
            output_lines.append(colorize(line, 'green', color))
        else:
            output_lines.append(line)

    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            f.write('\n'.join(output_lines))
        print(f"Diff saved to {args.output}")
    else:
        print('\n'.join(output_lines))

if __name__ == '__main__':
    main()
