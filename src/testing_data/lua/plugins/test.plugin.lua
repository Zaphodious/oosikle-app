Plugin:Credit({
    authors = { "HotFish", "Croug" },
    version = 1,
    year = 2025
})

Plugin:DeclareMediaCategory {
    name = "videogame",
    types = {
        { name = "pico8",
            file_extensions = { "p8", "png", "p8.png" } }
    },
}

Plugin:ViewAdapter {
    name="general_vg_view_adapter",
    media_category = "videogame",
    initial_page_sql = [[select * from objects where objects.collection_uuid=:coll_uuid]],
    views = {
        {
            viewtype = "table",
            initial_view_fn = function()
                -- just imagine an html-returning function here
            end,
            update_view_fn = function()
                -- imagine another one here
            end,
            assemble_object = function()
                -- Takes in a row table from page_sql and spits out a table with the expected fields.
                -- Optional. If doesn't exist, the row will just be passed into the 
                -- view function as-is.
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
            assemble_object = function()
                -- imagine similar to above here. each viewtype gets its own 
                -- assemble_object function as different displays might need 
                -- different information
            end,
        }
    },
}

Plugin:ObjectAdapter {
    name = "pico8_adapter",
    media_type = "pico8",
    custom_detail_renderer = function()
        -- emits HTML for the detail view
    end,
    play_button_action = function()
        -- somehow, handle a 'play' button press
    end,
}
