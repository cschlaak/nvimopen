## About The Project

Sometimes you have to work with Windows but still want to use your favorite editor, neovim.
This little project will help you integrate neovim into your daily Windows Explorer usage, by allowing to double-click files and open them in a new neovim tab.
Of course, this tool can also be used on Linux.

## Getting Started

### Installation

1. Make sure [Neovim](https://github.com/neovim/neovim) and [Neovide](https://github.com/neovide/neovide) are installed
2. Add the directories with your Neovim and Neovide binaries to your PATH variable
3. Install Rust via [rustup](https://rustup.rs) for example
4. Clone the repo
   ```sh
   git clone https://github.com/cschlaak/nvimopen.git
   ```
5. Build the project
   ```sh
   cargo build
   ```
6. Copy the produced `nvimopen` binary to a place where you (and your PATH variable) can find it (*optional*)

### Usage

- Open a single file
  ```sh
  nvimopen file.txt
  ```
- Open a file and jump to a line
  ```sh
  nvimopen file.txt:22
  ```
- Open multiple files
  ```sh
  nvimopen file1.txt:22 file2.txt
  ```

### Explorer Setup

- open file properties and select default application (nvimopen.exe) under "Opens with"

  <img src="/images/explorer-file-properties.png" alt="explorer file properties" width="350"/>

- your explorer will now show the corresponding icon on files that are opened with nvimopen, and thus in a new tab of your running neovim instance:

  <img src="/images/explorer-files-preview.png" alt="explorer file properties" width="500"/>

