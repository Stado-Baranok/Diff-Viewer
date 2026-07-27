// diff_viewer.cpp
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <map>
#include <cstring>
#include <diff_match_patch.h>
#include <unistd.h>

using namespace std;

bool colorEnabled = false;

string colorize(const string& text, const string& color) {
    if (!colorEnabled) return text;
    map<string, string> codes = {
        {"red", "\033[91m"},
        {"green", "\033[92m"},
        {"blue", "\033[94m"},
        {"reset", "\033[0m"}
    };
    return codes[color] + text + codes["reset"];
}

vector<string> readLines(const string& path) {
    ifstream file(path);
    vector<string> lines;
    string line;
    if (!file) {
        cerr << "Error reading " << path << endl;
        exit(1);
    }
    while (getline(file, line)) {
        lines.push_back(line);
    }
    return lines;
}

int main(int argc, char* argv[]) {
    string file1, file2, output;
    int context = 3;
    bool ignoreSpace = false;
    bool unified = true;
    bool forceColor = false;

    for (int i = 1; i < argc; ++i) {
        string arg = argv[i];
        if (arg == "--context" && i+1 < argc) context = stoi(argv[++i]);
        else if (arg == "--ignore-space") ignoreSpace = true;
        else if (arg == "--unified" && i+1 < argc) unified = (string(argv[++i]) == "true");
        else if (arg == "--color") forceColor = true;
        else if (arg == "--output" && i+1 < argc) output = argv[++i];
        else if (arg == "--help") {
            cout << "Usage: " << argv[0] << " <file1> <file2> [options]\n";
            return 0;
        } else {
            if (file1.empty()) file1 = arg;
            else if (file2.empty()) file2 = arg;
            else cerr << "Unknown argument: " << arg << endl;
        }
    }

    if (file1.empty() || file2.empty()) {
        cerr << "Two files are required." << endl;
        return 1;
    }

    colorEnabled = forceColor || isatty(STDOUT_FILENO);

    auto lines1 = readLines(file1);
    auto lines2 = readLines(file2);
    if (ignoreSpace) {
        for (auto& l : lines1) l = l.erase(l.find_last_not_of(" \t")+1);
        for (auto& l : lines2) l = l.erase(l.find_last_not_of(" \t")+1);
    }

    diff_match_patch dmp;
    // Объединяем строки в один текст с разделителями
    string text1, text2;
    for (const auto& l : lines1) text1 += l + "\n";
    for (const auto& l : lines2) text2 += l + "\n";

    auto diffs = dmp.diff_main(text1, text2, false);
    dmp.diff_cleanupSemantic(diffs);

    vector<string> outputLines;
    outputLines.push_back(colorize("--- " + file1, "blue"));
    outputLines.push_back(colorize("+++ " + file2, "blue"));

    for (const auto& d : diffs) {
        string text = d.text;
        // Удаляем лишние переводы строк для красоты (упрощённо)
        if (d.operation == DIFF_DELETE) {
            outputLines.push_back(colorize("-" + text, "red"));
        } else if (d.operation == DIFF_INSERT) {
            outputLines.push_back(colorize("+" + text, "green"));
        } else {
            outputLines.push_back(" " + text);
        }
    }

    string result;
    for (const auto& l : outputLines) result += l + "\n";

    if (!output.empty()) {
        ofstream out(output);
        out << result;
        cout << "Diff saved to " << output << endl;
    } else {
        cout << result;
    }

    return 0;
}
