local plugin_def = {}

function Plugin(packagename: string)
    plugin_def.packagename = packagename
    return function(plugindef)
        return plugindef
    end
end

function Info(infotable)
    plugin_def.authors = infotable.authors
    plugin_def.version = infotable.version
    plugin_def.date = infotable.date
end

function Definitions(definitions)
    return definitions
end

function MediaCategory(category_name: string)
    local media_category_record = {media_category_key = category_name, media_category_string_key=""}
    local media_types = {}
    local extension_tags = {}
    return function(categorydef)
        return categorydef
    end
end

function ViewAdapter(adapterdef)
    return adapterdef
end

function ObjectAdapter(mediatype)
    return function(adapterinfo)
        return adapterinfo
    end
end

return plugin_def
