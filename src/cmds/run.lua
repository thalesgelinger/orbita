return {
    "run",
    params = { "file" },
    description = "Run the project with resolved dependencies",
    exec = function(args)
        local path = require "path"
        local deps = dofile(path.DEPENDENCY_FILE)

        for _, dep in pairs(deps.dependencies) do
            if dep.path then
                package.path = package.path .. ";" .. string.format("%s/?.lua", dep.path)
            end
            -- local package_path = string.format("%s/%s/%s/?.lua", path.ORBITE_HOME, repo:gsub("/", "-"), version)
            -- package.path = package.path .. ";" .. package_path
        end

        print("RUN THIS ", args[1])
    end
}
