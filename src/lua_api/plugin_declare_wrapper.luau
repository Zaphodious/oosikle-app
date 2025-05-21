local plugin_def = {}

local function Plugin(packagename: string)
    plugin_def.packagename = packagename
    return function(plugindef)
        return plugindef
    end
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

return plugin_def
