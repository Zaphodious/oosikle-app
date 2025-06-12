
local function __str_table_interop(s, tab)
    return (s:gsub('($%b{})', function(w) return tab[w:sub(3, -2)] or w end))
  end

getmetatable("").__mod = __str_table_interop
