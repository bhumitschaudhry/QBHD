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
            local client_id = vim.lsp.start({
                name = "qbhd-lsp",
                cmd = lsp_cmd,
                root_dir = vim.fs.dirname(vim.fs.find({ ".git", "*.bas" }, { upward = true })[1]),
            })

            if client_id then
                -- Get the actual client object from the ID
                local client = vim.lsp.get_client_by_id(client_id)

                -- Enhanced keymaps
                local buf = vim.api.nvim_get_current_buf()
                local map_opts = { buffer = buf, silent = true }

                vim.keymap.set("n", "gd", vim.lsp.buf.definition, map_opts)
                vim.keymap.set("n", "K", vim.lsp.buf.hover, map_opts)
                vim.keymap.set("n", "gr", vim.lsp.buf.references, map_opts)
                vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, map_opts)
                vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, map_opts)
                vim.keymap.set("i", "<C-Space>", vim.lsp.buf.completion, map_opts)

                -- Enhanced hover with border
                vim.lsp.handlers["textDocument/hover"] = vim.lsp.with(
                    vim.lsp.handlers.hover, {
                        border = "rounded",
                        max_width = 80,
                    }
                )

                -- Semantic highlighting (if supported)
                if client and client.server_capabilities
                    and client.server_capabilities.semanticTokensProvider then
                    vim.lsp.semantic_tokens.start(buf, client_id)
                end
            end
        end,
    })

    -- Commands
    vim.api.nvim_create_user_command("QBHDCompile", function()
        local file = vim.fn.expand("%")
        vim.cmd("!qbhd " .. vim.fn.shellescape(file))
    end, { desc = "Compile current file with QBHD" })

    vim.api.nvim_create_user_command("QBHDRun", function()
        local file = vim.fn.expand("%")
        local output = vim.fn.expand("%:r")
        vim.cmd("!qbhd " .. vim.fn.shellescape(file) .. " && " .. vim.fn.shellescape(output))
    end, { desc = "Compile and run current file" })

    vim.api.nvim_create_user_command("QBHDCheck", function()
        local file = vim.fn.expand("%")
        vim.cmd("!qbhd --check " .. vim.fn.shellescape(file))
    end, { desc = "Check current file for errors" })

    vim.api.nvim_create_user_command("QBHDFormat", function()
        -- Basic formatting: trim trailing whitespace, ensure final newline
        local buf = vim.api.nvim_get_current_buf()
        local lines = vim.api.nvim_buf_get_lines(buf, 0, -1, false)
        local formatted = {}
        for _, line in ipairs(lines) do
            -- Trim trailing whitespace
            table.insert(formatted, line:match("^(.-)%s*$"))
        end
        -- Ensure file ends with newline
        if #formatted > 0 and formatted[#formatted] ~= "" then
            table.insert(formatted, "")
        end
        vim.api.nvim_buf_set_lines(buf, 0, -1, false, formatted)
        vim.notify("QBHD: Formatted", vim.log.levels.INFO)
    end, { desc = "Format current BASIC file" })

    vim.api.nvim_create_user_command("QBHDInfo", function()
        local client_id = vim.lsp.get_clients({ name = "qbhd-lsp", bufnr = 0 })
        if #client_id > 0 then
            vim.notify("QBHD LSP: Connected (client ID: " .. client_id[1].id .. ")", vim.log.levels.INFO)
        else
            vim.notify("QBHD LSP: Not connected", vim.log.levels.WARN)
        end
    end, { desc = "Show QBHD LSP connection info" })
end

return M
