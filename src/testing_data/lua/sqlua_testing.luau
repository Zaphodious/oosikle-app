
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

function SQLuaAddsMediaCategory(category_id, category_key)
    --DB:upsert_media_category_record({media_category_id=category_id, media_category_string_key=category_key})
    local q2 = DB:query([[insert into MediaCategories values (?1, ?2)]], {'foob', 'foob_key'})
    local query_res = DB:query([[select * from MediaCategories M where M.media_category_id=?;]], {category_id})[1]
    return query_res.media_category_string_key
end

function SerdeWorksAsExpected(category_id)
    local query_res = DB:query([[select * from MediaCategories M where M.media_category_id=? limit 1;]], {category_id})[1]
    return query_res
end

function Foo()
    print("hello")
end

print(SQLuaFetches)
