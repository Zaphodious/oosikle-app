Plugin:Credit({
    authors = { "HotFish", "Croug" },
    version = 1,
    year = 2025
})

Plugin:MediaCategory {
    name = "videogame",
    types = {
        { name = "pico8",
            file_extensions = { "p8", "png", "p8.png" } }
    },
}

Plugin:ViewAdapter({
    media_category = "videogame",
    views = {
        {
            viewtype = "table",
            initial_view_fn = function()
                -- just imagine an html-returning function here
            end,
            update_view_fn = function()
                -- imagine another one here
            end,

        },
        {
            viewtype = "card",
            initial_view_fn = function()
                -- just imagine an html-returning function here
            end,
            update_view_fn = function()
                -- imagine another one here
            end,
        }
    },
    headers = {
        { "title",   "object_name" },
        { "developer", "object_artist" }
    },
    page_sql = [[select * from objects where objects.collection_uuid=:coll_uuid]],
})
