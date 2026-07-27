// diff_viewer.cs
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using DiffPlex;
using DiffPlex.DiffBuilder;
using DiffPlex.DiffBuilder.Model;

class DiffViewer
{
    private static bool colorEnabled;

    static void Main(string[] args)
    {
        var file1 = "";
        var file2 = "";
        int context = 3;
        bool ignoreSpace = false;
        bool unified = true;
        bool forceColor = false;
        string output = null;

        for (int i = 0; i < args.Length; i++)
        {
            switch (args[i])
            {
                case "--context": context = int.Parse(args[++i]); break;
                case "--ignore-space": ignoreSpace = true; break;
                case "--unified": unified = bool.Parse(args[++i]); break;
                case "--color": forceColor = true; break;
                case "--output": output = args[++i]; break;
                case "--help":
                    Console.WriteLine("Usage: dotnet run -- <file1> <file2> [options]");
                    return;
                default:
                    if (file1 == "") file1 = args[i];
                    else if (file2 == "") file2 = args[i];
                    else Console.WriteLine($"Unknown argument: {args[i]}");
                    break;
            }
        }

        if (file1 == "" || file2 == "")
        {
            Console.Error.WriteLine("Two files are required.");
            Environment.Exit(1);
        }

        colorEnabled = forceColor || !Console.IsOutputRedirected;

        var lines1 = File.ReadAllLines(file1);
        var lines2 = File.ReadAllLines(file2);
        if (ignoreSpace)
        {
            lines1 = lines1.Select(l => l.TrimEnd()).ToArray();
            lines2 = lines2.Select(l => l.TrimEnd()).ToArray();
        }

        var diffBuilder = new InlineDiffBuilder(new Differ());
        var diffResult = diffBuilder.BuildDiffModel(string.Join("\n", lines1), string.Join("\n", lines2));

        var outputLines = new List<string>();
        outputLines.Add(Colorize($"--- {file1}", "blue"));
        outputLines.Add(Colorize($"+++ {file2}", "blue"));

        // Упрощённый вывод построчно
        foreach (var line in diffResult.Lines)
        {
            switch (line.Type)
            {
                case ChangeType.Inserted:
                    outputLines.Add(Colorize($"+{line.Text}", "green"));
                    break;
                case ChangeType.Deleted:
                    outputLines.Add(Colorize($"-{line.Text}", "red"));
                    break;
                default:
                    outputLines.Add($" {line.Text}");
                    break;
            }
        }

        string result = string.Join("\n", outputLines);
        if (output != null)
        {
            File.WriteAllText(output, result);
            Console.WriteLine($"Diff saved to {output}");
        }
        else
        {
            Console.WriteLine(result);
        }
    }

    static string Colorize(string text, string color)
    {
        if (!colorEnabled) return text;
        var codes = new Dictionary<string, string>
        {
            ["red"] = "\x1b[91m",
            ["green"] = "\x1b[92m",
            ["blue"] = "\x1b[94m",
            ["reset"] = "\x1b[0m"
        };
        return codes[color] + text + codes["reset"];
    }
}
