<div align="center">

<img src="docs/banner.png" alt="RepoWiki — 为任意代码库生成 wiki 文档" width="100%">

[![GitHub](https://img.shields.io/badge/github-UnitBuilds--CC/RepoWiki-blue.svg)](https://github.com/UnitBuilds-CC/RepoWiki)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/UnitBuilds-CC/RepoWiki/actions/workflows/ci.yml/badge.svg)](https://github.com/UnitBuilds-CC/RepoWiki/actions/workflows/ci.yml)

[**快速开始**](#快速开始) · [**工作原理**](#工作原理) · [English](README.md)

</div>

**开源 DeepWiki 替代品** — 从终端或浏览器为任意代码仓库生成完整 wiki 文档。

在线演示是 RepoWiki 吃自家狗粮的产物：用 `repowiki scan . --site` 扫描本仓库生成，部署在 GitHub Pages。

## 为什么选 RepoWiki？

| | DeepWiki | deepwiki-open | **RepoWiki** |
|---|---------|--------------|-------------|
| 部署方式 | SaaS，不可自托管 | Docker Compose | **`cargo install repowiki`** |
| 本地仓库 | 不支持 | 不支持 | **原生支持** |
| CLI | 无 | 无 | **有** |
| Web UI | 有 | 有 | **有** |
| 导出格式 | 仅网页 | 仅网页 | **Markdown / JSON / HTML** |
| 阅读指南 | 无 | 无 | **PageRank 排名 + 阅读路径** |
| 终端问答 | 无 | 无 | **`repowiki chat`** |
| 依赖 | N/A | Docker + PostgreSQL | **Rust + SQLite** |

## 快速开始

```bash
cargo install repowiki

# 设置 API Key（DeepSeek、OpenAI、Anthropic、OpenRouter 等）
export DEEPSEEK_API_KEY=<your-api-key>
# 或者
repowiki config set api_key <your-api-key>

# 扫描本地项目
repowiki scan ./my-project

# 扫描 GitHub 仓库
repowiki scan https://github.com/pallets/flask

# 扫描私有 GitHub 仓库（token 不会落进日志或报错）
GITHUB_TOKEN=ghp_xxx repowiki scan https://github.com/acme/private-repo

# 生成自包含 HTML 并打开
repowiki scan ./my-project --format html --open

# 启动 Web 界面
repowiki serve ./my-project   # 可选：启动时直接加载一个项目
```

扫描时会遵守 `.gitignore` 和 `.repowikiignore`，并默认跳过 `.env`、`.env.local`、`.npmrc`、`.pypirc`、SSH 私钥等本地敏感文件，避免把不该进入文档上下文的内容喂给后续分析。

## 核心功能

- **索引驱动分析** — 确定性的预索引（符号、import、调用图）为 LLM 提供紧凑的结构化摘要，而非直接扔原始源码；每个模块的 token 开销降低约 85%，同时描述仍基于真实代码。
- **结构化 wiki** — 项目概览、逐模块文档、自动识别的架构（含 Mermaid 图），以及基于 PageRank 的"从这里开始读"路径。
- **页面交叉链接**：页面里反引号包住的符号名或文件路径，如果正好对应另一个 wiki 页面，就会自动变成指向那一页的链接（Markdown 里是相对 .md 链接，HTML 导出里是页内跳转）。围栏代码块原样保留；同一个名字被多个页面定义时，链到第一个页面。
- **全局符号索引**：索引页汇总分析记录的所有关键符号，先按类别（class、function 等）分组，组内再按模块归类，每个符号都链回所属模块页；没有记录符号的项目不生成该页。
- **增量重跑**：输出目录里的 `.repowiki-state.json` 记录每个页面由哪些输入生成，再次扫描只重新生成源码有变化的页面，并清理被删模块对应的页面；JSON 和 HTML 导出在内容没有变化时直接不写盘。加 `--full` 可强制全量重建。没变的页面连 LLM 调用都省掉：分析结果存在按内容寻址的 SQLite 缓存里（`~/.repowiki/cache.db`），小改之后重扫对没动的模块零 API 调用。想提交后自动刷新，在 `.git/hooks/post-commit` 里触发一次 `repowiki scan . --site -o docs/wiki &`，或在 CI 里 push 后跑一次——缓存保证了它足够便宜，不需要常驻监听进程。
- **import 感知排名** — 先解析 Python、JS/TS、Go、Rust、Java、C/C++ 的 import 再排名，并跳过 minified/生成式 bundle，避免浪费 LLM 上下文。
- **覆盖率如实标注** — 扫不下整个仓库时绝不装成扫完了：概览页和 CLI 都会明确标出部分覆盖（实际纳入 vs 候选文件数、超大文件与被排除目录），wiki 不会悄悄自称完整。
- **三种导出格式** — 可直接提交的 Markdown 目录、结构化 JSON，或自包含、随手能分享的 HTML 单文件（含图表）。
- **静态站点发布**：`repowiki scan . --site` 会在 Markdown 导出目录里生成 docsify 加载页（`index.html` 和 `.nojekyll`），把目录推到 GitHub Pages 上就是一个能直接浏览的文档站。
- **Web 查看器 + 终端问答**：三栏浏览器界面，或 `repowiki chat .` 在终端里做基于源码的问答。问答支持多轮对话：之前的问答会带进每次请求，Web 界面和 CLI 里都能追问。内置 TF-IDF 检索（无需 embedding 服务），索引会落盘缓存，对没动过的仓库第二次启动直接热启动，不用重建。
- **CLI 优先** — 单个静态二进制，无运行时、无 Docker、无数据库、无浏览器依赖。

```bash
repowiki scan .                    # 生成 wiki
repowiki scan . --full             # 忽略增量状态，全量重建每个页面
repowiki scan . -f html --open     # 浏览器打开
repowiki scan . -l zh              # 中文输出
repowiki index .                   # 仅预索引，不调 LLM（符号、import、调用图）
repowiki chat .                    # 多轮追问的代码问答，本次会话有记忆
repowiki map .                     # 按真实依赖排序的仓库地图，零 LLM 调用
repowiki map . --format json       # 给 agent 用的可入 prompt 排序清单
repowiki scan . --site             # 在 Markdown 导出基础上上生成 GitHub Pages 加载页
```

## 语言与模型

识别 Python、JavaScript、TypeScript、Go、Rust、Java、Kotlin、C/C++、C#、Ruby、PHP、Swift 等 30+ 种语言。支持 DeepSeek、OpenAI、Anthropic、Google Gemini、OpenRouter 以及任何 OpenAI 兼容端点：

```bash
repowiki config set model deepseek   # deepseek / claude / gpt / gemini / openrouter/...
repowiki scan . -m gpt               # 或直接传模型名
```

## 配置

RepoWiki 按以下顺序查找配置：
1. 命令行参数（`-m`、`-l`、`-o`）
2. 环境变量（`REPOWIKI_MODEL`、`REPOWIKI_API_KEY`）
3. 配置文件（`~/.repowiki/config.json`）
4. 各提供商专用环境变量（`DEEPSEEK_API_KEY`、`OPENROUTER_API_KEY`、`OPENAI_API_KEY`、`ANTHROPIC_API_KEY`）

私有 GitHub 仓库用 `GITHUB_TOKEN`（或 `GH_TOKEN`）：clone 走认证连接，token 只出现在 git 调用内部，不落日志、不进报错。

## 项目结构

```
RepoWiki/
├── crates/
│   ├── core/          # 共享模型、配置、错误类型
│   ├── scanner/       # 文件扫描 + 语言识别
│   ├── graph/         # import 解析 + PageRank
│   ├── index/         # 符号提取 + 调用图
│   ├── cache/         # SQLite 按内容寻址缓存
│   ├── llm/           # 多提供商 HTTP 客户端
│   ├── analyzer/      # 索引驱动的 LLM 分析流水线
│   ├── wiki/          # Wiki 页面组装
│   ├── rag/           # 面向问答的 TF-IDF 检索
│   ├── export/        # Markdown / JSON / HTML 导出
│   ├── ingest/        # 本地目录 + GitHub clone
│   ├── server/        # Axum web 后端
│   └── cli/           # CLI 入口
├── frontend/          # React + Vite + TailwindCSS
├── Cargo.toml
└── LICENSE
```

## 工作原理

![RepoWiki 流程](docs/architecture.png)

1. **扫描** — 遍历目录树，过滤二进制、生成式 bundle 和超大文件，检测语言和入口文件
2. **索引** — 提取符号、解析 6+ 种语言的 import、构建调用图、跑 PageRank——全部确定性，零 LLM 调用
3. **分析** — 把紧凑的索引摘要而非原始源码喂给 LLM，分 4 步完成（概览、模块、架构、阅读指南）
4. **缓存** — SQLite 按内容 hash 缓存，重新扫描时跳过未变更文件
5. **导出** — 组装 wiki 页面，注入 Mermaid 图和源码链接，按选定格式输出

## 开发

```bash
git clone https://github.com/UnitBuilds-CC/RepoWiki.git
cd RepoWiki

# 构建
cargo build --workspace

# 前端（可选，用于 Web UI）
cd frontend && npm install && npm run dev

# 运行
cargo run -- serve --port 8000
```

## 后续规划

生成、Web 界面、图表这几块已经能用，页面之间互相链接，重跑只重新生成源码有变化的页面，`scan --site` 还能一键导出可直接部署 GitHub Pages 的静态站点。接下来主要是更丰富的图表：

- **更多图表类型**：在依赖图之外再加调用图和数据流图——分析本来就走了 import，能挖出更多。

## 许可证

MIT
