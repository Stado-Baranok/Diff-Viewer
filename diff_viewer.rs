// diff_viewer.rs
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use structopt::StructOpt;
use similar::{Algorithm, TextDiff};

#[derive(Debug, StructOpt)]
#[structopt(name = "diff_viewer")]
struct Opt {
    file1: String,
    file2: String,
    #[structopt(long, default_value = "3")]
    context: usize,
    #[structopt(long)]
    ignore_space: bool,
    #[structopt(long)]
    unified: bool,
    #[structopt(long)]
    color: bool,
    #[structopt(long)]
    output: Option<String>,
}

fn colorize(text: &str, color: &str, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }
    let code = match color {
        "red" => "\x1b[91m",
        "green" => "\x1b[92m",
        "blue" => "\x1b[94m",
        _ => "",
    };
    format!("{}{}\x1b[0m", code, text)
}

fn main() {
    let opt = Opt::from_args();
    let content1 = fs::read_to_string(&opt.file1).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", opt.file1, e);
        std::process::exit(1);
    });
    let content2 = fs::read_to_string(&opt.file2).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", opt.file2, e);
        std::process::exit(1);
    });

    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();

    // Игнорирование пробелов: просто trim_end
    let lines1_trimmed: Vec<String> = if opt.ignore_space {
        lines1.iter().map(|s| s.trim_end().to_string()).collect()
    } else {
        lines1.iter().map(|s| s.to_string()).collect()
    };
    let lines2_trimmed: Vec<String> = if opt.ignore_space {
        lines2.iter().map(|s| s.trim_end().to_string()).collect()
    } else {
        lines2.iter().map(|s| s.to_string()).collect()
    };

    let diff = TextDiff::configure()
        .algorithm(Algorithm::Myers)
        .newlines(false)
        .diff_lines(&lines1_trimmed, &lines2_trimmed);

    let mut output = Vec::new();
    // Вывод в стиле unified (упрощённо)
    // Заголовки
    output.push(colorize(&format!("--- {}", opt.file1), "blue", opt.color));
    output.push(colorize(&format!("+++ {}", opt.file2), "blue", opt.color));

    for group in diff.grouped_ops(opt.context) {
        let (old_start, old_len) = (group[0].old_range().start, group[0].old_range().len());
        let (new_start, new_len) = (group[0].new_range().start, group[0].new_range().len());
        let header = format!("@@ -{},{} +{},{} @@", old_start+1, old_len, new_start+1, new_len);
        output.push(colorize(&header, "blue", opt.color));

        for op in group {
            for line in diff.iter_changes(op).unwrap() {
                let sign = if line.old_index().is_some() && line.new_index().is_some() {
                    " "
                } else if line.old_index().is_some() {
                    "-"
                } else {
                    "+"
                };
                let text = line.to_string();
                let colored = if sign == "-" {
                    colorize(&format!("{}{}", sign, text), "red", opt.color)
                } else if sign == "+" {
                    colorize(&format!("{}{}", sign, text), "green", opt.color)
                } else {
                    format!(" {}", text)
                };
                output.push(colored);
            }
        }
    }

    let out = output.join("\n");
    if let Some(out_file) = opt.output {
        fs::write(&out_file, out).unwrap();
        println!("Diff saved to {}", out_file);
    } else {
        println!("{}", out);
    }
}
