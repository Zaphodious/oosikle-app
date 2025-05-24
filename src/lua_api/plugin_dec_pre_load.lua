return function(plugin_function)
    local plugin_def
    local fn_ret = plugin_function(plugin_def)
    if fn_ret == nil then
        return plugin_def
    else
        return fn_ret
    end
end
