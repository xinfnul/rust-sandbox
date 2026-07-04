# todo-app

A simple command-line todo manager written in Rust. Tasks can be added, listed, marked as completed, deleted, and automatically persisted between runs using a plain text storage file.

---

## Project Structure

```text
todo-app/
├── src/
│   └── main.rs         # application implementation
├── Cargo.toml          # project manifest
├── Cargo.lock
└── todos.txt           # created automatically after first save
```

---

## Requirements

- Rust (stable toolchain)
- Cargo

---

## Building

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

---

## Running

```bash
cargo run
```

The application presents an interactive menu:

```text
1. Add Todo
2. List Todos
3. Remove Todo
4. Toggle Completed
5. Exit
6. Cleanup
```

---

## Features

- Add new tasks
- View all todos
- Toggle completed/pending status
- Delete tasks by ID
- Persistent storage between runs
- Colored terminal output
- Cleanup command to remove all saved data

---

## Usage

Start the application:

```bash
cargo run
```

Example session:

```text
1. Add Todo
> Enter todo name: Learn Rust ownership

2. List Todos

ID     : 1
Task   : Learn Rust ownership
Status : [PENDING]

4. Toggle Completed
> Enter todo id: 1

2. List Todos

ID     : 1
Task   : Learn Rust ownership
Status : [DONE]
```

---

## Storage Format

Todos are stored in a plain text file named `todos.txt`.

Each line follows this format:

```text
id|task|completed
```

Example:

```text
1|Learn Rust|true
2|Build CLI|false
3|Read The Book|true
```

The application loads this file automatically on startup and saves changes after every modification.

---

## How It Works

1. On startup, the application loads `todos.txt` if it exists.
2. Each todo is parsed into a `Todo` struct containing:
   - unique ID
   - task name
   - completion status
3. New todos receive an incrementing ID.
4. Every add, delete, or status change immediately updates the storage file.
5. The cleanup option removes all todos and deletes the storage file.
6. Colored terminal output improves readability using the `colored` crate.

---

## Error Handling

The application gracefully handles common failures:

- Missing storage file
- Invalid menu input
- Invalid todo IDs
- Corrupted storage lines (ignored during loading)
- File I/O errors when saving

Example:

```text
> Enter todo id: 99

No todo found with that id.
```

---

## Dependencies

| Crate     | Purpose                 |
| --------- | ----------------------- |
| `colored` | Colored terminal output |

---

## Limitations

- Stores data in a local text file only.
- Single-user application.
- Interactive terminal interface only.
- No task editing, search, filtering, priorities, or due dates.
- No concurrent access protection.

---

*Built as part of my Rust learning journey to practice ownership, structs, file I/O, error handling, and building interactive CLI applications.*
