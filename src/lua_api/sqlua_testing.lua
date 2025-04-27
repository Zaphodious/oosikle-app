
TestReturn = "OK"

print("We loaded!")

function SQLuaFetches()
    return DB:query([[select * from Objects where Objects.object_uuid='DEADBEEFDEADBEEFDEADBEEFDEADBEEF';]], {})[1].object_name
end

function SQLuaCreatesHTML(collection_uuid)
    local query_res = DB:query([[select O.* from Objects O inner join ObjectsInCollections OC on O.object_uuid=OC.object_uuid where OC.collection_uuid = ?1;]], {collection_uuid})
end

function Foo()
    print("hello")
end

print(SQLuaFetches)
