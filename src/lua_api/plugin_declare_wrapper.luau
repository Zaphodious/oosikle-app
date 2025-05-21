function parse_plugin_dec()
    local plugin_def = {}

    local function RegisterPlugin(plugindef)
        print("we are registering!")
        print(plugindef)
        plugin_def = plugindef
    end

    local function Plugin(packagename: string)
        plugin_def.packagename = packagename
        return function(plugindef)
            return plugindef
        end
    end

    local function CheckPlugin()
        return plugin_def
    end

    local function Info(infotable)
        plugin_def.authors = infotable.authors
        plugin_def.version = infotable.version
        plugin_def.date = infotable.date
    end

    local function Definitions(definitions)
        return definitions
    end

    local function MediaCategory(category_name: string)
        local media_category_record = {media_category_key = category_name, media_category_string_key=""}
        local media_types = {}
        local extension_tags = {}
        return function(categorydef)
            return categorydef
        end
    end

    local function ViewAdapter(adapterdef)
        return adapterdef
    end

    local function ObjectAdapter(mediatype)
        return function(adapterinfo)
            return adapterinfo
        end
    end

    --insert_plugin_def_here--

    return CheckPlugin()
end 
