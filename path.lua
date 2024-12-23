local lfs = require "lfs"
local json = require "cjson"
local Path = {}

Path.ORBITE_HOME = os.getenv("ORBITE_HOME") or os.getenv("HOME") .. "/.orbite"
Path.INDEX_FILE = Path.ORBITE_HOME .. "/index.json"
Path.DEPENDENCY_FILE = "orb.lua"

function Path.ensure_orbite_directory()
    if not lfs.attributes(Path.ORBITE_HOME, "mode") then
        os.execute("mkdir -p " .. Path.ORBITE_HOME)
    end
end

-- Load global index file
function Path.load_global_index()
    local f = io.open(Path.INDEX_FILE, "r")
    if not f then return {} end
    local data = f:read("*a")
    f:close()
    return json.decode(data)
end

return Path
