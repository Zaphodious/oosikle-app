
TestReturn = "OK"

print("We loaded!")

function SQLuaFetches()
    return SQL:query([[select * from Objects where Objects.object_uuid='DEADBEEFDEADBEEFDEADBEEFDEADBEEF';]], {})[1].object_name
end

function Foo()
    print("hello")
end

print(SQLuaFetches)
