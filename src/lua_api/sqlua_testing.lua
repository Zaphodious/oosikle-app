
TestReturn = "OK"

print("We loaded!")

function SQLuaFetches()
    print("We're starting the function")
    print(SQLua)
    SQLua.set_query([[select * from Objects where Objects.id=X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF';]])
    print("We have set the query")
    --[[
    local ret = SQLua.query([])
    print("We have called the query")
    TestReturn = ret[0].object_name
    print("And we should be done")
    print(TestReturn)
    ]]
end

function Foo()
    print("hello")
end

print(sqlua_fetches)

