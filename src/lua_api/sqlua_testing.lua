
TestReturn = "OK"

print("We loaded!")

function SQLuaFetches()
    return SQL:query([[select * from Objects where Objects.object_uuid='DEADBEEFDEADBEEFDEADBEEFDEADBEEF';]], {})[1].object_name
end

function dump(o)
   if type(o) == 'table' then
      local s = '{ '
      for k,v in pairs(o) do
         if type(k) ~= 'number' then k = '"'..k..'"' end
         s = s .. '['..k..'] = ' .. dump(v) .. ','
      end
      return s .. '} '
   else
      return tostring(o)
   end
end


function Foo()
    print("hello")
end

print(SQLuaFetches)
