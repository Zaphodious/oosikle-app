Plugin "oosikle.builtin.pico8" {

    Info {
        authors = { "HotFish", "Croug" },
        version = 1,
        date = "2025-05-04"
    },

    Definitions {
        MediaCategory "videogame" {
            pico8 = { "p8.png", "p8", "png" }
        },
    },

    ViewAdapter {
        media_type = "pico8",
        page_sql = [[select * from Objects o inner join ObjectsInCollections oc on o.object_uuid=oc.object_uuid where oc.collection_uuid=:collection_uuid order by oc.index_in_collection;]],
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
    },

    ObjectAdapter {
        media_type = "pico8",
        custom_detail_view = function(object_uuid, settings) end,
        play_action = function(object_uuid, settings)
            return { action = "run", exe = "path_from_settings", args = "run=path_to_p8_file" }
        end,
        initialize_object = function(file_table, settings) end,
        settings = {
            pico8path = { type = "filepath", default = nil }
        },
    }
}
