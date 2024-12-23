return {
    "install",
    params = { "repo", "version" },
    description = "Install a specific package from GitHub",
    exec = function(args)
        local socket = require "socket.http"
        local json = require "cjson"
        local path = require "path"

        local repo, version = args[1], args[2]
        path.ensure_orbite_directory()
        local index = path.load_global_index()

        if index[repo] and index[repo][version] then
            print(string.format("%s version %s is already installed", repo, version))
            return
        end


        local function fetch_package_from_github(repo, version)
            local url = string.format("https://raw.githubusercontent.com/%s/%s/init.lua", repo, version)
            local response, status = socket.request(url)
            if status ~= 200 then
                error("Failed to fetch package from GitHub repo: " .. repo .. " version: " .. version)
            end
            return response
        end
        print(string.format("Fetching %s version %s from GitHub...", repo, version))
        local package_data = fetch_package_from_github(repo, version)

        local package_dir = string.format("%s/%s/%s", path.ORBITE_HOME, repo:gsub("/", "-"), version)
        os.execute("mkdir -p " .. package_dir)

        local f = io.open(package_dir .. "/init.lua", "w")
        f:write(package_data)
        f:close()

        index[repo] = index[repo] or {}
        index[repo][version] = package_dir
        local function save_global_index(index)
            local f = io.open(path.INDEX_FILE, "w")
            f:write(json.encode(index))
            f:close()
        end
        save_global_index(index)

        print(string.format("Installed %s version %s", repo, version))
    end
}
