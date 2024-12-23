-- Orbite: Lua Package Manager
-- This implementation uses GitHub repositories directly for package management.
package.cpath = package.cpath .. ";/Users/tgelin01/.asdf/installs/lua/5.1/luarocks/lib/lua/5.1/?.so"
package.path = package.path ..
    ";/Users/tgelin01/.luarocks/share/lua/5.1/?.lua;/Users/tgelin01/.luarocks/share/lua/5.1/?/init.lua"
package.cpath = package.cpath .. ";/Users/tgelin01/.luarocks/lib/lua/5.1/?.so"

local orb = {}
local lfs = require "lfs"            -- LuaFileSystem for directory operations
local socket = require "socket.http" -- LuaSocket for HTTP requests

-- Global Constants
local ORBITE_HOME = os.getenv("ORBITE_HOME") or os.getenv("HOME") .. "/.orbite"
local INDEX_FILE = ORBITE_HOME .. "/index.json"
local DEPENDENCY_FILE = "orb.lua"

-- Ensure orbite directory exists
local function ensure_orbite_directory()
    if not lfs.attributes(ORBITE_HOME, "mode") then
        os.execute("mkdir -p " .. ORBITE_HOME)
    end
end

-- Load global index file
local function load_global_index()
    local f = io.open(INDEX_FILE, "r")
    if not f then return {} end
    local data = f:read("*a")
    f:close()
    return json.decode(data)
end

-- Save global index file
local function save_global_index(index)
    local f = io.open(INDEX_FILE, "w")
    f:write(json.encode(index))
    f:close()
end

-- Fetch a package from a GitHub repository
local function fetch_package_from_github(repo, version)
    local url = string.format("https://raw.githubusercontent.com/%s/%s/init.lua", repo, version)
    local response, status = socket.request(url)
    if status ~= 200 then
        error("Failed to fetch package from GitHub repo: " .. repo .. " version: " .. version)
    end
    return response
end

-- Install a package
function orb.install(repo, version)
    ensure_orbite_directory()
    local index = load_global_index()

    -- Check if the package is already installed
    if index[repo] and index[repo][version] then
        print(string.format("%s version %s is already installed", repo, version))
        return
    end

    -- Fetch and install the package
    print(string.format("Fetching %s version %s from GitHub...", repo, version))
    local package_data = fetch_package_from_github(repo, version)

    local package_dir = string.format("%s/%s/%s", ORBITE_HOME, repo:gsub("/", "-"), version)
    os.execute("mkdir -p " .. package_dir)

    local f = io.open(package_dir .. "/init.lua", "w")
    f:write(package_data)
    f:close()

    -- Update index
    index[repo] = index[repo] or {}
    index[repo][version] = package_dir
    save_global_index(index)

    print(string.format("Installed %s version %s", repo, version))
end

-- Resolve dependencies from orb.lua
function orb.resolve_dependencies()
    local deps = dofile(DEPENDENCY_FILE)
    if not deps.dependencies then
        error("Invalid dependency file")
    end

    ensure_orbite_directory()
    local index = load_global_index()

    for repo, version in pairs(deps.dependencies) do
        if not (index[repo] and index[repo][version]) then
            orb.install(repo, version)
        end
    end
end

-- Run a project with resolved dependencies
function orb.run(entry_file)
    local deps = dofile(DEPENDENCY_FILE)

    -- Update package.path
    for repo, version in pairs(deps.dependencies) do
        local package_path = string.format("%s/%s/%s/?.lua", ORBITE_HOME, repo:gsub("/", "-"), version)
        package.path = package.path .. ";" .. package_path
    end

    -- Execute the entry file
    dofile(entry_file)
end

-- CLI Commands
local function main()
    local cmd = arg[1]
    for i, value in pairs(arg) do
        print("I: ", i, "V: ", value)
    end
    if cmd == "install" then
        orb.install(arg[2], arg[3])
    elseif cmd == "resolve" then
        orb.resolve_dependencies()
    elseif cmd == "run" then
        orb.run(arg[2])
    else
        print("Usage: orb <command> [args]")
        print("Commands:")
        print("  install <repo> <version>   Install a specific package from GitHub")
        print("  resolve                    Resolve project dependencies")
        print("  run <entry_file>           Run the project with resolved dependencies")
    end
end

main()
