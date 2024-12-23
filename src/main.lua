-- Orbite: Lua Package Manager
-- This implementation uses GitHub repositories directly for package management.
package.cpath = package.cpath .. ";/Users/tgelin01/.asdf/installs/lua/5.1/luarocks/lib/lua/5.1/?.so"
package.path = package.path ..
    ";/Users/tgelin01/.luarocks/share/lua/5.1/?.lua;/Users/tgelin01/.luarocks/share/lua/5.1/?/init.lua"
package.cpath = package.cpath .. ";/Users/tgelin01/.luarocks/lib/lua/5.1/?.so"

local function help()
    print("Usage: orb <command> [args]")

    local cmds = require("cmds.init")
    local help_tbl = {}

    local max_usage_length = 0
    for _, value in pairs(cmds) do
        local usage = "\t" .. value[1]
        if value.params then
            for _, param in ipairs(value.params) do
                usage = string.format("%s <%s>", usage, param)
            end
        end
        max_usage_length = math.max(max_usage_length, #usage)
    end

    for _, value in pairs(cmds) do
        local usage = "\t" .. value[1]
        if value.params then
            for _, param in ipairs(value.params) do
                usage = string.format("%s <%s>", usage, param)
            end
        end

        local description = value.description or ""
        local padding = string.rep(" ", max_usage_length - #usage)

        table.insert(help_tbl, { usage .. padding, description })
    end

    for _, entry in ipairs(help_tbl) do
        print(entry[1] .. "  " .. entry[2])
    end
end

local function main()
    local cmd = arg[1] or ""

    local status, fn = pcall(require, "cmds." .. cmd)

    if not status then
        help()
        return
    end
    table.remove(arg, 1)
    fn.exec(arg)
end

main()
