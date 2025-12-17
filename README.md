# 大作业：Wordle 🦀

2024 年夏季学期《程序设计训练》 Rust 课堂大作业（一）。

## 🌟 项目特色 (Project Features)

本项目不仅实现了 Wordle 游戏的基础功能，还通过丰富的交互模式、智能求解算法以及灵活的配置选项，极大地增强了可玩性和技术深度。

### 1. 🎮 多样化的交互模式
*   **命令行交互模式 (CLI)**：经典的终端交互体验，使用彩色文本直观展示猜测结果（Green 🟩 / Yellow 🟨 / Red 🟥）。
*   **文本用户界面 (TUI)**：基于 `tui-rs` 和 `crossterm` 构建的图形化终端界面，提供输入框、状态展示和实时的按键响应，支持键盘操作，带来类似原生应用的体验。

### 2. 🧠 智能求解器与提示系统
*   **基于信息熵的推荐**：内置基于信息论（Information Theory）的求解算法，能够计算当前状态下每个候选词的信息熵。
*   **多维度提示**：
    *   **🔍 Most Informative Guesses**：推荐能最大程度缩减搜索空间（即信息熵最大）的单词，帮助快速排除错误选项。
    *   **🎯 Best Guesses**：推荐在当前候选集中概率最高的单词，直接冲刺答案。
    *   **🌐 Global Optimum**：通过多步前瞻（Lookahead）算法，计算全局最优的猜测路径。

### 3. ⚙️ 丰富的游戏模式与配置
*   **每日挑战**：支持通过 `--day` 和 `--seed` 参数指定随机种子，生成固定的每日谜题，方便与朋友进行同题竞技。
*   **困难模式**：通过 `-D` 或 `--difficult` 开启，强制要求后续猜测必须包含已知的线索，增加游戏挑战性。
*   **统计功能**：通过 `--stats` 记录并展示胜率、平均猜测次数以及常用词频统计。
*   **存档机制**：支持 JSON 格式的游戏状态保存与加载，随时中断并恢复游戏进度。
*   **灵活配置**：支持命令行参数与 JSON 配置文件混合使用，方便自定义词库（`--final-set`, `--acceptable-set`）和游戏参数。

## 🛠 实现细节 (Implementation Details)

### 核心逻辑与状态管理
*   **Word 结构体**：使用 `Word` 结构体封装单词逻辑，内部使用哈希表 (`HashMap<char, Vec<u8>>`) 存储字母位置信息，实现了高效的 `compare` 方法来生成猜测反馈（2=Green, 1=Yellow, 0=Red），确保了核心规则判定的准确性与性能。
*   **状态机设计**：游戏主循环采用状态机模式处理用户输入、游戏判定和状态转换，无论是 CLI 还是 TUI 模式均复用了底层的游戏逻辑 (`game.rs`)，保证了行为的一致性。

### 智能求解算法
*   **信息熵计算**：求解器核心在于计算每个候选词的**香农熵 (Shannon Entropy)**。
    $$ E[I] = \sum_{x \in Outcomes} -p(x) \log_2 p(x) $$
    其中 $p(x)$ 是在做出某个猜测后，得到特定反馈（如 G-Y-X-X-X）的概率。熵越高，意味着该猜测平均能消除的不确定性越多。
*   **并行加速**：利用 `rayon` 库将候选词的信息熵计算任务分发到多核 CPU 上并行执行，显著减少了计算等待时间。
*   **Beam Search (束搜索)**：为了寻找全局最优解（Global Optimum），算法不仅仅看当前一步，而是使用 Beam Search 算法进行多步推演。维护一个大小为 10 的优先队列（Binary Heap），在搜索空间中寻找未来期望熵最大的猜测路径。

### TUI 界面实现
*   使用 `tui-rs` 库构建布局，将屏幕划分为输入区、输出区和键盘区。
*   使用 `crossterm` 监听键盘事件（Char, Backspace, Enter, Esc），实现了非阻塞的实时输入响应。
*   通过 `Spans` 和 `Style` 动态渲染彩色文本，实时反馈猜测结果和键盘状态。

## 其他说明

1. `src/builtin_words` 是内嵌于程序中的单词列表，`FINAL` 为所有答案词，`ACCEPTABLE` 为所有候选词。
2. 为了实现更多功能（如 GUI 或求解器），你可以自由地调整本项目的结构（如增加新的 binary 或者划分 crate，或者使用 Cargo workspace 组织多级项目），但需要满足以下条件，并在验收时提前告知助教：
    * 所有的测试命令都能够按现有的方式运行；
    * 不能对 `tests` 目录的内容进行任何修改（但可以整体移动到某个位置）。

## 作业要求

具体要求请查看[作业文档](https://lab.cs.tsinghua.edu.cn/rust/projects/wordle/background/)。

## Honor Code

请在 `HONOR-CODE.md` 中填入你完成作业时参考的内容，包括：

* 开源代码仓库（直接使用 `crate` 除外）
* 查阅的博客、教程、问答网站的网页链接
* 与同学进行的交流

## 自动测试

本作业的基础要求部分使用 Cargo 进行自动化测试，运行 `cargo test [--release] -- --test-threads=1` 即可运行测试。其中 `[--release]` 的意思是可以传 `--release` 参数也可以不传，例如 `cargo test -- --test-threads=1` 表示在 debug 模式下进行单线程测试，而 `cargo test --release -- --test-threads=1` 表示在 release 模式下进行单线程此时。

如果某个测试点运行失败，将会打印 `case [name] incorrect` 的提示（可能会有额外的 `timeout` 提示，可以忽略）。你可以在 `tests/cases` 目录下查看测试用例的内容，还可以使用以下命令手工测试：

```bash
cp tests/cases/[case_name].before.json tests/data/[case_name].run.json # 复制游戏初始状态文件（如果需要）
cargo run [--release] -- [options] < test/cases/[case_name].in > test/cases/[case_name].out # 运行程序
diff tests/cases/[case_name].ans tests/cases/[case_name].out # 比较输出
jq -set tests/data/[case_name].after.json tests/data/[case_name].run.json # 比较游戏状态文件（如果需要）
```

其中 `[options]` 是游戏使用的命令行参数，`[case_name]` 是测试用例的名称。`jq` 工具可以使用各类包管理器（如 `apt` 或 `brew`）安装。

项目配置了持续集成（CI）用于帮助你测试。在推送你的改动后，可以在 GitLab 网页上查看 CI 结果和日志。

---

# 🇬🇧 English Description

This is the Final Project (I) for "Programming Practice" in Rust, Summer Semester 2024.

## 🌟 Project Features

This project implements a full-featured Wordle game in Rust, enhanced with rich interaction modes, intelligent solving algorithms, and flexible configurations.

### 1. 🎮 Diverse Interaction Modes
*   **CLI Mode**: Classic terminal experience with colored text output (Green 🟩 / Yellow 🟨 / Red 🟥) representing game feedback.
*   **TUI Mode**: A graphical terminal user interface built with `tui-rs` and `crossterm`, featuring input boxes, status displays, and real-time keyboard response for a native app-like experience.

### 2. 🧠 Intelligent Solver & Hint System
*   **Entropy-Based Suggestions**: Incorporates an algorithm based on Information Theory to calculate the entropy of each candidate word.
*   **Multi-Dimensional Hints**:
    *   **Most Informative Guesses**: Suggests words that maximize information gain (entropy), helping to narrow down the search space efficiently.
    *   **Best Guesses**: Suggests the most probable answers from the remaining candidate set.
    *   **Global Optimum**: Uses a multi-step lookahead algorithm to find the optimal guessing path strategy.

### 3. ⚙️ Game Modes & Configuration
*   **Daily Challenge**: Generate consistent puzzles using `--day` and `--seed` arguments, allowing competition on the same word.
*   **Hard Mode**: Enabled via `-D` or `--difficult`, enforcing strict rules where subsequent guesses must respect revealed hints.
*   **Statistics**: Tracks win rates, average guess counts, and word frequency via `--stats`.
*   **Save/Load State**: Supports saving and loading game progress in JSON format.
*   **Flexible Config**: Supports both command-line arguments and JSON configuration files, allowing customization of word lists (`--final-set`, `--acceptable-set`) and game parameters.

## 🛠 Implementation Details

### Core Logic & State Management
*   **Word Struct**: Encapsulates word logic using `HashMap<char, Vec<u8>>` to store letter positions. The efficient `compare` method generates feedback (Green/Yellow/Red), ensuring accurate and fast rule evaluation.
*   **State Machine**: The main loop handles user input and state transitions. Both CLI and TUI modes share the underlying game logic (`game.rs`) for consistency.

### Intelligent Solver Algorithm
*   **Entropy Calculation**: The core of the solver calculates **Shannon Entropy** for candidate words.
    $$ E[I] = \sum_{x \in Outcomes} -p(x) \log_2 p(x) $$
    Higher entropy means the guess reduces more uncertainty on average.
*   **Parallel Computing**: Utilizes the `rayon` library to parallelize entropy calculations across multiple CPU cores, significantly improving performance.
*   **Beam Search**: To find the Global Optimum, the solver employs a Beam Search algorithm (keeping the top 10 paths in a Binary Heap) to perform a multi-step lookahead, optimizing the guessing strategy over several turns.

### TUI Implementation
*   Built with `tui-rs` to organize the terminal screen into input, output, and keyboard sections.
*   Uses `crossterm` for non-blocking event handling (Char, Backspace, Enter, Esc).
*   Dynamically renders colored text using `Spans` and `Style` to provide immediate visual feedback.

## Other Notes

1. `src/builtin_words` is the embedded word list; `FINAL` contains all answer words, and `ACCEPTABLE` contains all candidate words.
2. To implement more features (like GUI or solver), you may adjust the project structure (e.g., adding binaries or crates, using Cargo workspace), provided that:
    * All test commands run as currently specified.
    * The content of the `tests` directory remains unmodified (though the directory itself can be moved).

## Assignment Requirements

For specific requirements, please refer to the [Assignment Document](https://lab.cs.tsinghua.edu.cn/rust/projects/wordle/background/).

## Honor Code

Please fill in `HONOR-CODE.md` with the references used while completing the assignment, including:
* Open-source code repositories (excluding direct use of `crate`).
* Links to blogs, tutorials, and Q&A websites consulted.
* Communication with classmates.

## Auto Testing

The basic requirements of this assignment use Cargo for automated testing. Run `cargo test [--release] -- --test-threads=1` to execute tests. `[--release]` means you can optionally pass the `--release` flag (e.g., `cargo test -- --test-threads=1` for single-threaded testing in debug mode, `cargo test --release -- --test-threads=1` for release mode).

If a test case fails, a message like `case [name] incorrect` will be printed (ignore potential `timeout` warnings). You can check test cases in the `tests/cases` directory and manually test using:

```bash
cp tests/cases/[case_name].before.json tests/data/[case_name].run.json # Copy initial game state (if needed)
cargo run [--release] -- [options] < test/cases/[case_name].in > test/cases/[case_name].out # Run program
diff tests/cases/[case_name].ans tests/cases/[case_name].out # Compare output
jq -set tests/data/[case_name].after.json tests/data/[case_name].run.json # Compare game state (if needed)
```

`[options]` are command-line arguments, and `[case_name]` is the test case name. `jq` can be installed via package managers like `apt` or `brew`.

The project is configured with CI to help you test. After pushing changes, check CI results and logs on the GitLab webpage.
