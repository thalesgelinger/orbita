return {
    "run",
    params = { "file" },
    description = "Run the project with resolved dependencies",
    exec = function(file)
        local path = require "path"
        local deps = dofile(path.DEPENDENCY_FILE)

        for repo, version in pairs(deps.dependencies) do
            local package_path = string.format("%s/%s/%s/?.lua", path.ORBITE_HOME, repo:gsub("/", "-"), version)
            package.path = package.path .. ";" .. package_path
        end

        dofile(file)
    end
}
