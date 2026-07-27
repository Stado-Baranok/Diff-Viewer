📊 Diff Viewer – Построчное сравнение файлов
Мощный консольный инструмент для сравнения текстовых файлов с цветной подсветкой, поддержкой контекста и гибкими настройками.
Реализован на 7 языках программирования – выберите свой!

✨ Возможности
🔍 Построчное сравнение – показывает добавленные, удалённые и изменённые строки.

🎨 Цветная подсветка – зелёным (+) добавления, красным (–) удаления, синим для метаданных.

📏 Контекст – возможность задать количество строк контекста вокруг изменений (опция --context).

⚙️ Игнорирование пробелов – опция --ignore-space для сравнения без учёта пробелов в конце строк.

📄 Унифицированный формат – вывод в стиле diff -u (с заголовками файлов и номерами строк).

💾 Сохранение результата – опция --output для записи diff в файл.

🌐 Кроссплатформенность – работает в Linux, macOS и Windows.

⚡ Быстрая работа – эффективные алгоритмы для больших файлов.

📦 Поддерживаемые языки
Язык	Версия	Файл	Основная библиотека
Python	3.8+	diff_viewer.py	difflib (встроенная)
Go	1.18+	diff_viewer.go	github.com/sergi/go-diff
Rust	1.60+	diff_viewer.rs	similar
JavaScript	Node.js 14+	diff_viewer.js	diff
C#	.NET 6+	diff_viewer.cs	DiffPlex
Java	11+	DiffViewer.java	diffutils / google-diff-match-patch
C++	C++17	diff_viewer.cpp	diff-match-patch
🚀 Быстрый старт
1. Склонируйте репозиторий
bash
git clone https://github.com/yourname/diff-viewer.git
cd diff-viewer
2. Запустите на любом языке
Python

bash
python diff_viewer.py file1.txt file2.txt --context 3 --ignore-space --color
Go

bash
go mod init diff_viewer
go get github.com/sergi/go-diff/diffmatchpatch
go run diff_viewer.go file1.txt file2.txt -c 3 -i -color
Rust (сборка)

bash
cargo new diff_viewer
# добавьте зависимости в Cargo.toml
cargo run -- file1.txt file2.txt --context 3 --ignore-space --color
JavaScript (Node.js)

bash
npm install diff
node diff_viewer.js file1.txt file2.txt --context 3 --ignore-space --color
C#

bash
dotnet new console -n diff_viewer
dotnet add package DiffPlex
dotnet run -- file1.txt file2.txt --context 3 --ignore-space --color
Java (сборка с Maven/Gradle)

bash
javac -cp .:diffutils.jar DiffViewer.java
java -cp .:diffutils.jar DiffViewer file1.txt file2.txt --context 3 --ignore-space --color
C++ (сборка с diff-match-patch)

bash
g++ -std=c++17 -I/usr/include/diff-match-patch diff_viewer.cpp -ldiff_match_patch -o diff_viewer
./diff_viewer file1.txt file2.txt --context 3 --ignore-space --color
📋 Пример вывода
Для файлов old.txt и new.txt:

old.txt:

text
Hello
World
Foo
Bar
new.txt:

text
Hello
World
Baz
Qux
Вывод (с цветами):

text
--- old.txt
+++ new.txt
@@ -1,4 +1,4 @@
 Hello
 World
-Foo
-Bar
+Baz
+Qux
Цвета: - красный, + зелёный, заголовки и метаданные синие.

⚙️ Опции командной строки
Флаг	Описание
--context <N>	Количество строк контекста (по умолчанию 3)
--ignore-space	Игнорировать пробелы в конце строк
--unified	Вывод в унифицированном формате (включён по умолчанию)
--color	Принудительно включить цветной вывод
--output <file>	Сохранить diff в файл (иначе stdout)
--help	Справка
📄 Лицензия
MIT – свободно используйте, модифицируйте и распространяйте.

🤝 Вклад
Приветствуются pull request'ы! Если хотите добавить новый язык или улучшить существующий – создавайте issue.

🧠 Авторы
Проект создан в образовательных целях для демонстрации алгоритмов сравнения на разных языках
