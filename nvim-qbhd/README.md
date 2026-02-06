# nvim-qbhd

Neovim plugin for QBHD/QB64 BASIC with LSP support.

## Installation

### lazy.nvim

```lua
{
    "qbhd/nvim-qbhd",
    ft = "basic",
    config = function()
        require("qbhd").setup({
            cmd = { "/path/to/qbhd-lsp" }
        })
    end
}
```

### packer.nvim

```lua
use {
    "qbhd/nvim-qbhd",
    ft = "basic",
    config = function()
        require("qbhd").setup()
    end
}
```

## Features

- Real-time diagnostics
- Syntax checking
- LSP integration

## Configuration

```lua
require("qbhd").setup({
    cmd = { "qbhd-lsp" },  -- Path to LSP server
})
```

## Requirements

- Neovim >= 0.8
- qbhd-lsp binary in PATH
