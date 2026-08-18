# rust-sandbox

A personal sandbox for learning and exploring Rust — built incrementally as I dive deeper into the language and ecosystem. Each project is self-contained with its own README.

---

## Projects

### CLI Applications

| Project                                       | Description                                                       |
| --------------------------------------------- | ----------------------------------------------------------------- |
| [`todo-app`](./todo-app/)                     | Command-line todo manager with persistent task storage            |
| [`password-generator`](./password-generator/) | Secure password generator with customizable options               |
| [`rgrep`](./rgrep/)                           | Minimal `grep` clone for searching text using regular expressions |
| [`markdown`](./markdown/)                     | Simple Markdown parser and renderer                               |
| [`log-analyzer`](./log-analyzer/)             | CLI tool for parsing and analyzing log files                      |
| [`threadpool`](./threadpool/)                 | Threadpool implementation with concurrent image downloader        |
| [`async-rust`](./async-rust/)                 | Async image downloader with Rust async/await and an async runtime|

---

## Structure

```text
rust-sandbox/
├── todo-app/            # command-line todo manager
├── password-generator/  # secure password generator
├── rgrep/               # grep-like search tool
├── markdown/            # markdown parser
├── log-analyzer/        # log parsing and analysis ( cargo workspaces )
├── threadpool/          # thread-based concurrent image downloader
├── async-rust/          # async concurrent image downloader
└── README.md            # you are here
```

Each subdirectory has its own `README.md` with build instructions, usage examples, and implementation notes.

---

_This repo grows as I learn. Implementations may be rough — that's the point._
