
TestReturn = "OK"

print("We loaded!")

function SQLuaFetches()
    print("We're starting the function")
    print(SQL)
    SQL:with_sql([[select * from Objects where Objects.object_uuid=?1;]])
    print("We have set the query")
    local ret = SQL:query({[[DEADBEEFDEADBEEFDEADBEEFDEADBEEF]]})
    print("We have called the query")
    for k,v in pairs(ret) do
        print(k.." = "..v)
    end
    print(ret)
    TestReturn = ret.get(0).object_name
    print("And we should be done")
    print(TestReturn)
end

function Foo()
    print("hello")
end

print(SQLuaFetches)

