<p align="center">
    <img src="media/logo.png" alt="titta" width="200"/>
</p>
  
<p align="center">
  <em>An alternative to ls built in Rust.</em>
</p>
  
<p align="center">
    <img src="https://img.shields.io/crates/v/titta?style=flat-square&color=blueviolet&link=https%3A%2F%2Fcrates.io%2Fcrates%2Ftitta" alt="Crates.io version" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-stable-orange?style=flat-square" alt="Rust" />
  <img src="https://img.shields.io/github/last-commit/simon-danielsson/titta/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#info">Info</a> •
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#license">License</a>
</p>  
   

<!-- <p align="center"> -->
<!--   <img src="media/1.gif" alt="screenshot"> -->
<!-- </p> -->

---
<div id="info"></div>

## 📌 Information
  
Titta is a rust alternative to the ls unix tool.
  
---
<div id="install"></div>

## 📦 Install
    
``` bash
cargo install titta
```
   
---
<div id="usage"></div>

## 💻 Usage
    
``` bash
$ ta <flags> <optional path>
```
  
### Flags
  
``` bash
-i : devicons
-w : color
-a : show hidden files
-e : show which .sh files are executable with '*' suffix
```
  
### Subcommands
  
``` bash
tree <level> : view as tree hierarchy
    example usage:
    $ ta tree 3 -i -a ~/Downloads/

help : view available flags, subcommands etc.
    example usage:
    $ ta help
```
   
---
<div id="license"></div>

## 📜 License
This project is licensed under the [MIT License](https://github.com/simon-danielsson/titta/blob/main/LICENSE).  
