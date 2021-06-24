<h1 align="center"> ReVi </h1>
<p align="center">
  <a><img alt="MAINTAINED" src="https://img.shields.io/badge/Maintained%3F-yes-green.svg"></a>
  <a><img alt="Downloads" src="https://img.shields.io/crates/d/revi"></a>
  <a href="https://crates.io/crates/revi"><img alt="crates.io" src="https://img.shields.io/crates/v/revi.svg"></a>
  <a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
</p>
<p align="center">
  <a><img alt="Stars" src="https://img.shields.io/github/stars/revi-editor/revi?style=social"></a>
  <a><img alt="Forks" src="https://img.shields.io/github/forks/revi-editor/revi?style=social"></a>
  <a><img alt="watchers" src="https://img.shields.io/github/watchers/revi-editor/revi?style=social"></a>
  <a><img alt="contributors" src="https://img.shields.io/github/contributors/revi-editor/revi"></a>
</p>
<p align="center">
  <a><img alt="issues" src="https://img.shields.io/github/issues/revi-editor/revi"></a>
  <a><img alt="last commit" src="https://img.shields.io/github/last-commit/revi-editor/revi"></a>
  <a><img alt="repo size" src="https://img.shields.io/github/repo-size/revi-editor/revi"></a> <a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
</p>
<p align="center">
  <a><img alt="RUST" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"></a>
  <a><img alt="LUA" src="https://img.shields.io/badge/Lua-2C2D72?style=for-the-badge&logo=lua&logoColor=white"></a>
</p>

# Table Of Contents:

  - [**About**](#about)
  - [**Usage**](#usage)
    - [**Install**](#cratesio)
    - [**Clone && Installing**](#github)
    - [**Development**](#development-use)
  - [**KeyBindings**](#keybindings)
  - [**Road Map**](#road-map)
  - [**Change Log**](./change_log.md)

# About

ReVi is a cross-platform terminal based Vim inspired text editor.
Currently ReVi is in heavy development probably not good to use for every day use
but I have been using this editor to while working on this project to help find bugs.
If you like what you see help the project out with a [github](https://github.com/revi-editor/revi) star.
If you find a bug please feel free to open a issues for it.

<p align="center">
  <a><img alt="Image" src="./snapshots/line_numbers.png"></a>
</p>


# Usage

### **Crates.io**
```sh
cargo install revi --version="0.0.0-beta-0.5"
revi <filename>
```
### **GitHub**
```sh
git clone https://github.com/revi-editor/revi
cd revi
cargo install --path .
revi <filename>
```

### **Development Use**
```sh
git clone https://github.com/revi-editor/revi
cd revi
cargo run --release -- <filename>
```

# KeyBindings

### **Normal Mode**

|'Key'|*Command*|Note
|:---|:---:|:---:
`Esc`|*NormalMode*|
`ZZ`|*Save Quit*|
`ZQ`|*Quit*|
`<C-y>`|*ScrollUp Cursor Keeps Line Number*|Not working 100% correct
`<C-e>`|*ScrollDown Cursor Keeps Line Number*|Not working 100% correct
`<C-u>`|*ScrollUp*|Not working 100% correct
`<C-d>`|*ScrollDown*|Not working 100% correct
`j`|*CursorDown*|
`Down`|*CursorDown*|
`k`|*CursorUp*|
`Up`|*CursorUp*|
`h`|*CursorLeft*|
`Left`|*CursorLeft*|
`l`|*CursorRight*|
`Right`|*CursorRight*|
`:`|*CommandMode*|
`i`|*InsertMode*|
`x`|*DeleteChar*|
`Delete`|*DeleteChar*|
`d`|*DeleteLine*|
`Home`|*Home*|
`End`|*End*|
`0`|*Home*|
`$`|*End*|
`A`|*End InsertMode CursorLeft*|

### **Insert Mode**

|'Key'|*Command*|Note
|:---|:---:|:---:
`Esc`|*Normal*|
`Backspace`|*Backspace*|
`Enter`|*NewLine*|
`Home`|*Home*|
`End`|*End*|
`Down`|*CursorDown*|
`Up`|*CursorUp*|
`Left`|*CursorLeft*|
`Right`|*CursorRight*|

### **Command Mode**

|'Key'|*Command*|Note
|:---|:---:|:---:
`Esc`|*Normal*|
`Enter`|*Normal*|

# Road Map

- [ ] **Added Modes**:
  - [X] **Normal**
  - [X] **Insert**
  - [ ] **Command**
  - [ ] **Visual**
  - [ ] **Visual Line**
  - [ ] **Visual Block**
- [X] **Basic KeyBindings**
- [ ] **Basic Unicode Support**
- [ ] **Plugin API**:
  - [ ] **Custom KeyBindings**
- [ ] **Help Docs**
- [ ] **WebSite**
- [ ] **Package Manager**
- [ ] **LSP**
