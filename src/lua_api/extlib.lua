
local function __str_table_interop(s, tab)
    return (s:gsub('($%b{})', function(w) return tab[w:sub(3, -2)] or w end))
  end

getmetatable("").__mod = __str_table_interop

function Plugin(packagename)
  return function(plugindef)
    return plugindef
  end
end

function Info(infotable)
  return infotable
end

function Definitions(definitions)
  return definitions
end

function MediaCategory(category_name)
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

