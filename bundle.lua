local lfs = require("lfs") -- LuaFileSystem for file and directory handling

-- Bundling logic to inject into the output file
local BUNDLE_LOGIC = [[
local ____bundle__funcs, ____bundle__files, ____bundle__global_require = {}, {}, require
local require = function(path)
    if ____bundle__files[path] then
        return ____bundle__files[path]
    elseif ____bundle__funcs[path] then
        ____bundle__files[path] = ____bundle__funcs[path]()
        return ____bundle__files[path]
    end
    return ____bundle__global_require(path)
end
]]

-- Helper to read the entire contents of a file
local function read_file(file_path)
    local file, err = io.open(file_path, "r")
    if not file then
        error("Failed to open file: " .. file_path .. "\n" .. err)
    end

    local content = file:read("*a")
    file:close()
    return content
end

-- Helper to write to a file
local function write_file(file_path, content)
    local file, err = io.open(file_path, "w")
    if not file then
        error("Failed to create output file: " .. file_path .. "\n" .. err)
    end

    file:write(content)
    file:close()
end

-- Helper to recursively search for Lua files in a directory
local function find_lua_files(dir)
    local files = {}
    for file in lfs.dir(dir) do
        if file ~= "." and file ~= ".." then
            local path = dir .. "/" .. file
            local attr = lfs.attributes(path)

            if attr.mode == "file" and path:match("%.lua$") then
                table.insert(files, path)
            elseif attr.mode == "directory" then
                for _, nested_file in ipairs(find_lua_files(path)) do
                    table.insert(files, nested_file)
                end
            end
        end
    end
    return files
end

-- Extract module names from `require` calls
local function extract_requirements(lua_code)
    local requirements = {}
    for module in lua_code:gmatch('require%s*%(?%s*[\'"](.-)[\'"]%s*%)?') do
        table.insert(requirements, module)
    end
    return requirements
end

-- Resolve module paths relative to the source directory
local function resolve_module_path(module, source_dir)
    local path = module:gsub("%.", "/") .. ".lua"
    local full_path = source_dir .. "/" .. path
    return full_path
end

-- Add all Lua files in a folder to the bundle
local function include_folder(folder_path, source_dir, packages)
    for _, file_path in ipairs(find_lua_files(folder_path)) do
        -- Derive the module name from the file path
        local relative_path = file_path:sub(#source_dir + 2):gsub("%.lua$", "")
        local module_name = relative_path:gsub("/", ".")
        packages[module_name] = file_path
    end
end

-- Main bundling function
local function bundle_files(output_file, source_dir, main_file, auto_detect, packages, include_folders)
    -- Read main file
    local main_path = source_dir .. "/" .. main_file
    local main_contents = read_file(main_path)

    -- Detect required modules if auto-detect is enabled
    if auto_detect then
        for _, module in ipairs(extract_requirements(main_contents)) do
            local resolved_path = resolve_module_path(module, source_dir)
            if lfs.attributes(resolved_path, "mode") == "file" and not packages[module] then
                packages[module] = resolved_path
            end
        end
    end

    -- Include additional folders
    for _, folder in ipairs(include_folders) do
        include_folder(folder, source_dir, packages)
    end

    -- Prepare output contents
    local output_contents = BUNDLE_LOGIC

    -- Add packages to the output
    for module, package_path in pairs(packages) do
        local package_contents = read_file(package_path)
        output_contents = output_contents .. string.format(
            "____bundle__funcs[%q] = function()\n%s\nend\n",
            module,
            package_contents
        )
    end

    -- Add main file contents to the output
    output_contents = output_contents .. "\n" .. main_contents

    -- Write output
    write_file(output_file, output_contents)
    print("Bundling complete: " .. output_file)
end

local function main()
    local output_file = "build/orbite.lua"
    local source_dir = "src"
    local main_file = "main.lua"
    local auto_detect = true

    local packages = {
        ["path"] = source_dir .. "/path.lua"
    }

    local include_folders = {
        source_dir .. "/cmds",
    }

    bundle_files(output_file, source_dir, main_file, auto_detect, packages, include_folders)
end

main()
