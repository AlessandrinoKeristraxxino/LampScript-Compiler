# LampScript Language Support

This is the official VS Code extension for the LampScript programming language! It provides a rich development experience for anyone writing `.las` files.

## Features

- **Syntax Highlighting**: Comprehensive coloring for all LampScript keywords (`let`, `mod`, `fn`, `if`, `while`, `return`), built-in types (`u32`, `i32`, `f64`, `string`, etc.), and control flows.
- **Code Snippets**: Accelerate your coding with built-in snippets:
  - `let` -> Immutable variable declaration
  - `letm` -> Mutable variable declaration (`let name: mod u32 = value;`)
  - `fn` -> Function declaration
  - `if` / `while` -> Control flow structures
  - `print` / `println` -> Standard output statements
- **Bracket Matching**: Auto-closing and matching for `{ }`, `[ ]`, `( )`, and string quotes `" "`.

## Usage

1. Open a `.las` file.
2. The language mode will automatically switch to **LampScript**.
3. Enjoy the highlighted syntax and fast auto-completions using snippets!

## Requirements

If you have any requirements or dependencies, add a section describing those and how to install and configure them.

## Extension Settings

Include if your extension adds any VS Code settings through the `contributes.configuration` extension point.

For example:

This extension contributes the following settings:

* `myExtension.enable`: Enable/disable this extension.
* `myExtension.thing`: Set to `blah` to do something.

## Known Issues

Calling out known issues can help limit users opening duplicate issues against your extension.

## Release Notes

Users appreciate release notes as you update your extension.

### 1.0.0

Initial release of ...

### 1.0.1

Fixed issue #.

### 1.1.0

Added features X, Y, and Z.

---

## Working with Markdown

You can author your README using Visual Studio Code. Here are some useful editor keyboard shortcuts:

* Split the editor (`Cmd+\` on macOS or `Ctrl+\` on Windows and Linux).
* Toggle preview (`Shift+Cmd+V` on macOS or `Shift+Ctrl+V` on Windows and Linux).
* Press `Ctrl+Space` (Windows, Linux, macOS) to see a list of Markdown snippets.

## For more information

* [Visual Studio Code's Markdown Support](http://code.visualstudio.com/docs/languages/markdown)
* [Markdown Syntax Reference](https://help.github.com/articles/markdown-basics/)

**Enjoy!**
