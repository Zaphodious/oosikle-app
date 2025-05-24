return function()
    return {
    namespace = "oosikle.builtin.pico8",

    authors = { "HotFish", "Croug" },
    version = 1,
    date = "2025-05-04",

    definitions = {
        categories = {
            {
                media_category_id = "VIDEOGAME",
                media_category_string_key = "media_category_videogame",
            },
        },
        types = {
            {
                media_type_id = "PICO8",
                media_type_string_key = "media_type_pico8",
                media_category_id = "VIDEOGAME",
            }
        },
        file_extensions = {
            {
                file_extension_tag = "p8",
                file_extension_desc_string_key = "file_ext_p8",
            },
            {
                file_extension_tag = "p8.png",
                file_extension_desc_string_key = "file_ext_p8png",
            },
            {
                file_extension_tag = "png",
                file_extension_desc_string_key = "file_ext_png",
            },
        },
        types_for_file_extensions = {
            {
                file_extension_tag = "p8",
                media_type_id = "PICO8"
            },
            {
                file_extension_tag = "p8.png",
                media_type_id = "PICO8"
            },
            {
                file_extension_tag = "png",
                media_type_id = "PICO8"
            },
        }
    },

    view_adapters = {
        {
            media_category = "videogame",
            page_sql = [[select * from Objects o
                        inner join ObjectsInCollections oc on o.object_uuid=oc.object_uuid
                        where oc.collection_uuid=:collection_uuid
                        order by oc.index_in_collection;]],
            columns = {
                object_name = "title",
                object_artist = "developer",
                object_album_name = "series",
                object_album_position = "series_entry",
                object_imprint = "publisher",
                object_website = "bbs_link",
            },
            play_action = function(object, settings)
                Oosikle:open_link(object.object_website)
            end,
            settings = {
                skraper_api_key = { type = "string", default = nil }
            },
        }

    },
    object_adapters = {
        {
            media_type = "pico8",
            custom_detail_view = function(object_uuid, settings) end,
            play_action = function(object_uuid, settings)
                return { action = "run", exe = "path_from_settings", args = "run=path_to_p8_file" }
            end,
            initialize_object = function(file_table, settings) end,
            settings = {
                pico8path = { type = "filepath", default = nil }
            },
        },
    },

}
end
