return {
    "install",
    params = { "repo", "version" },
    description = "Install a specific package from GitHub",
    exec = function(...)
        print("INSTALL THIS", ...)
    end
}
