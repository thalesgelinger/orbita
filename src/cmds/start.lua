return {
    "start",
    description = "Start empty orbite project",
    exec = function(args)
        local lfs = require("lfs")
        local path = require("path")
        local current_directory = lfs.currentdir()

        local config = {
            name = current_directory,
            version = "1.0.0",
            description = "",
            main = "main.lua",
            dependencies = {},
            author = "",
            license = "ISC"
        }

        local function table_to_string(tbl)
            local str = "{\n"
            for k, v in pairs(tbl) do
                str = str .. "\t"
                if type(v) == "table" then
                    str = str .. k .. " = " .. table_to_string(v) .. ",\n"
                else
                    str = str .. k .. " = " .. string.format("\"%s\"", v) .. ",\n"
                end
            end
            if #str == 3 then
                return "{}"
            end
            str = str .. "}"
            return str
        end

        -- Function to write the table to a file
        local function write_table_to_file(filename, tbl)
            local file, err = io.open(filename, "w")
            if not file then
                print("Error opening file:", err)
                return
            end

            -- Write the table as `return { values }`
            file:write("return " .. table_to_string(tbl) .. "\n")

            file:close()
        end

        if args[1] == "-y" then
            write_table_to_file(path.DEPENDENCY_FILE, config)
            return
        end
        print "What is your project name?"
        config.name = io.read()

        print "What is your project description?"
        config.description = io.read()

        print "What is your project entry point?"
        config.main = io.read()

        print "Who is the author?"
        config.author = io.read()

        write_table_to_file(path.DEPENDENCY_FILE, config)
    end
}
