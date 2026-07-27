// DiffViewer.java
import java.io.*;
import java.nio.file.*;
import java.util.*;
import difflib.*;

public class DiffViewer {
    private static boolean color = true;

    public static void main(String[] args) throws Exception {
        String file1 = null, file2 = null, output = null;
        int context = 3;
        boolean ignoreSpace = false;
        boolean unified = true;
        boolean forceColor = false;

        for (int i = 0; i < args.length; i++) {
            switch (args[i]) {
                case "--context": context = Integer.parseInt(args[++i]); break;
                case "--ignore-space": ignoreSpace = true; break;
                case "--unified": unified = Boolean.parseBoolean(args[++i]); break;
                case "--color": forceColor = true; break;
                case "--output": output = args[++i]; break;
                case "--help":
                    System.out.println("Usage: java DiffViewer <file1> <file2> [options]");
                    return;
                default:
                    if (file1 == null) file1 = args[i];
                    else if (file2 == null) file2 = args[i];
                    else System.err.println("Unknown argument: " + args[i]);
            }
        }

        if (file1 == null || file2 == null) {
            System.err.println("Two files are required.");
            System.exit(1);
        }

        color = forceColor || System.console() != null;

        List<String> lines1 = Files.readAllLines(Paths.get(file1));
        List<String> lines2 = Files.readAllLines(Paths.get(file2));
        if (ignoreSpace) {
            lines1 = lines1.stream().map(s -> s.replaceAll("\\s+$", "")).toList();
            lines2 = lines2.stream().map(s -> s.replaceAll("\\s+$", "")).toList();
        }

        Patch patch = DiffUtils.diff(lines1, lines2);
        List<String> result = new ArrayList<>();
        result.add(colorize("--- " + file1, "blue"));
        result.add(colorize("+++ " + file2, "blue"));

        for (Delta delta : patch.getDeltas()) {
            // Упрощённый вывод: показываем удалённые и добавленные строки
            List<String> linesDel = delta.getOriginal().getLines();
            List<String> linesAdd = delta.getRevised().getLines();
            for (String l : linesDel) {
                result.add(colorize("-" + l, "red"));
            }
            for (String l : linesAdd) {
                result.add(colorize("+" + l, "green"));
            }
        }

        String out = String.join("\n", result);
        if (output != null) {
            Files.write(Paths.get(output), out.getBytes());
            System.out.println("Diff saved to " + output);
        } else {
            System.out.println(out);
        }
    }

    private static String colorize(String text, String colorName) {
        if (!color) return text;
        Map<String, String> codes = new HashMap<>();
        codes.put("red", "\033[91m");
        codes.put("green", "\033[92m");
        codes.put("blue", "\033[94m");
        codes.put("reset", "\033[0m");
        return codes.get(colorName) + text + codes.get("reset");
    }
}
