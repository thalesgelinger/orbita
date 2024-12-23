local lfs = require "lfs"
local Utils = {}
function Utils.impot_cmds(directory)
    local modules = {}

    for file in lfs.dir(directory) do
        if file ~= "." and file ~= ".." then
            local filePath = directory .. "/" .. file
            local mode = lfs.attributes(filePath, "mode")

            if mode == "file" and file:match("%.lua$") then
                local moduleName = directory .. "." .. file:sub(1, -5)
                local status, err = pcall(function()
                    table.insert(modules, require(moduleName))
                end)

                if not status then
                    print("Error loading module " .. moduleName .. ": " .. err)
                end
            end
        end
    end

    return modules
end

return Utils
