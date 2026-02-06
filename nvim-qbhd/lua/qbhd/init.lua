local M = {}

M.setup = function(opts)
    opts = opts or {}
    
    local lsp_cmd = opts.cmd or { "qbhd-lsp" }
    
    -- Enhanced diagnostics signs
    vim.fn.sign_define("DiagnosticSignError", { text = "✗", texthl = "DiagnosticSignError" })
    vim.fn.sign_define("DiagnosticSignWarn", { text = "⚠", texthl = "DiagnosticSignWarn" })
    vim.fn.sign_define("DiagnosticSignInfo", { text = "ℹ", texthl = "DiagnosticSignInfo" })
    vim.fn.sign_define("DiagnosticSignHint", { text = "💡", texthl = "DiagnosticSignHint" })
    
    -- Enhanced diagnostic config
    vim.diagnostic.config({
        virtual_text = {
            prefix = "●",
            spacing = 4,
        },
        signs = true,
        underline = true,
        update_in_insert = false,
        severity_sort = true,
        float = {
            border = "rounded",
            source = "always",
            header = "",
            prefix = "",
        },
    })
    
    vim.api.nvim_create_autocmd("FileType", {
        pattern = "basic",
        callback = function()
            local client = vim.lsp.start({
                name = "qbhd-lsp",
                cmd = lsp_cmd,
                root_dir = vim.fs.dirname(vim.fs.find({".git", "*.bas"}, { upward = true })[1]),
            })
            
            if client then
                -- Enhanced keymaps
                local buf = vim.api.nvim_get_current_buf()
                local opts = { buffer = buf, silent = true }
                
                vim.keymap.set("n", "gd", vim.lsp.buf.definition, opts)
                vim.keymap.set("n", "K", vim.lsp.buf.hover, opts)
                vim.keymap.set("n", "gr", vim.lsp.buf.references, opts)
                vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, opts)
                vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, opts)
                vim.keymap.set("i", "<C-Space>", vim.lsp.buf.completion, opts)
                
                -- Enhanced hover with border
                vim.lsp.handlers["textDocument/hover"] = vim.lsp.with(
                    vim.lsp.handlers.hover, {
                        border = "rounded",
                        max_width = 80,
                    }
                )
                
                -- Semantic highlighting
                if client and client.server_capabilities.semanticTokensProvider then
                    vim.lsp.semantic_tokens.start(buf, client)
                end
            end
        end,
    })
    
    -- Commands
    vim.api.nvim_create_user_command("QBHDCompile", function()
        vim.cmd("!qbhd %")
    end, {})
    
    vim.api.nvim_create_user_command("QBHDRun", function()
        local file = vim.fn.expand("%:r")
        vim.cmd("!qbhd % && ./" .. file)
    end, {})
    
    vim.api.nvim_create_user_command("QBHDCheck", function()
        vim.cmd("!qbhd --check %")
    end, {})
end

return M
