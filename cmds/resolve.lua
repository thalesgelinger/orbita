local path = require "path"
return {
    "resolve",
    description = "Resolve project dependencies",
    exec = function(...)
        local deps = dofile(path.DEPENDENCY_FILE)
        if not deps.dependencies then
            error("Invalid dependency file")
        end

        path.ensure_orbite_directory()
        local index = path.load_global_index()

        for repo, version in pairs(deps.dependencies) do
            if not (index[repo] and index[repo][version]) then
                require "cmds.install".exec({ repo, version })
            end
        end
    end
}
