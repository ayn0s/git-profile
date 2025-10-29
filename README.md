# gitp

A command-line tool for managing multiple Git configurations efficiently.

## Quick Install

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/ayn0s/git-profile/master/install.ps1 | iex
```

### Linux/macOS
```bash
curl -sSL https://raw.githubusercontent.com/ayn0s/git-profile/master/install.sh | bash
```

## Overview

`gitp` simplifies the management of multiple Git profiles by handling user configurations and SSH keys in a streamlined way. It's particularly useful for developers who need to switch between different Git identities for work, personal projects, or various organizations.

## Installation

### Pre-built binaries

Download the appropriate binary for your system from the [releases page](https://github.com/ayn0s/git-profile/releases):
- Linux: `gitp-linux-amd64`
- Windows: `gitp-windows-amd64.exe`

Add the binary to your system's PATH.

### From source

```bash
git clone https://github.com/ayn0s/git-profile
cd git-profile
cargo install --path .
```

## Usage

### Profile Management

Create a new profile:
```bash
gitp profile add
```

List existing profiles:
```bash
gitp profile list
```

Remove a profile:
```bash
gitp profile remove
```

### Repository Operations

Initialize a new repository:
```bash
gitp init [--name <directory_name>]
```

Clone a repository:
```bash
gitp clone <repository_url>
```

## License

MIT License - See [LICENSE](LICENSE) for details.