
TestReturn = "OK"

print("We loaded!")

function SQLuaFetches()
    return DB:query([[select * from Objects where Objects.object_uuid='DEADBEEFDEADBEEFDEADBEEFDEADBEEF';]], {})[1].object_name
end

function SQLuaCreatesHTMLBasic(collection_uuid)
    local query_res = DB:query([[select O.*, date(O.object_publish_timestamp) as pubdate from Objects O inner join ObjectsInCollections OC on O.object_uuid=OC.object_uuid where OC.collection_uuid = ?1;]], {collection_uuid})
    print(query_res)
    local ret = [[
            <table>
                <thead>
                    <tr>
                        <th>Title</th>
                        <th>Author</th>
                        <th>Portfolio</th>
                        <th>Publish Date</th>
                    </tr>
                </thead>
                <tbody>
                ]]
    for _, record in ipairs(query_res) do
        print(record)
        ret = ret .. ` <tr draggable="true"> <td>{record.object_name}</td> <td>{record.object_artist}</td> <td>{record.object_album_name}</td> <td>{record.pubdate}</td> </tr> `
    end
    local htmend =
                [[
                </tbody>
            </table>
    ]]

    return ret
end

function Foo()
    print("hello")
end

print(SQLuaFetches)
