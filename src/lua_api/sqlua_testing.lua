
TestReturn = "OK"

print("We loaded!")

function SQLuaFetches()
    print("We're starting the function")
    print(SQL)
    --SQL:with_sql([[select * from Objects where Objects.object_uuid='DEADBEEFDEADBEEFDEADBEEFDEADBEEF';]])
    --SQL:with_sql([[select * from Objects;]])
    SQL:with_sql([[select count(*) from Objects;]])
    print("We have set the query")
    local ret = SQL:query({})
    print("We have called the query")
    print(ret)
    --TestReturn = ret.get(1).object_name
    print(ret.get(0).count)
    print("And we should be done")
    print(TestReturn)
end

function Foo()
    print("hello")
end

print(SQLuaFetches)

