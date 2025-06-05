return function()
    return {
        namespace = "oosikle.builtin.simple_basic",

        authors = { "Person1", "Person2" },
        version = 1,
        date = "2025-05-25",

        definitions = {
            categories = {
                {
                    media_category_id = "BAZFILES",
                    media_category_string_key = "media_category_top",
                },
                {
                    media_category_id = "METADATA",
                    media_category_string_key = "media_category_metadata",
                }
            },
            types = {
                {
                    media_type_id = "FOODOC",
                    media_type_string_key = "media_type_foodoc",
                    media_category_id = "BAZFILES",
                },

                {
                    media_type_id = "FOOMETA",
                    media_type_string_key = "media_type_foometa",
                    media_category_id = "METADATA",
                }

            },
            file_extensions = {
                {
                    file_extension_tag = "fd.txt",
                    file_extension_desc_string_key = "file_ext_fdtxt",
                },
                {
                    file_extension_tag = "foo.txt",
                    file_extension_desc_string_key = "file_ext_footxt"

                },
                {
                    file_extension_tag = "foo",
                    file_extension_desc_string_key = "file_ext_foo",
                },
                {
                    file_extension_tag = "txt",
                    file_extension_desc_string_key = "file_ext_txt",
                },
                {
                    file_extension_tag = "foo.meta",
                    file_extension_desc_string_key = "file_ext_foometa"

                }
            },
            types_for_file_extensions = {
                {
                    file_extension_tag = "fd.txt",
                    media_type_id = "FOODOC"
                },
                {
                    file_extension_tag = "foo",
                    media_type_id = "FOODOC"
                },
                {
                    file_extension_tag = "txt",
                    media_type_id = "FOODOC"
                },
                {
                    file_extension_tag = "foo.meta",
                    media_type_id = "FOOMETA"
                }
            }
        },

        view_adapters = {
            {
                media_category = "BAZFILES",
                page_sql = [[select * from Objects o
                        inner join ObjectsInCollections oc on o.object_uuid=oc.object_uuid
                        where oc.collection_uuid=:collection_uuid
                        order by oc.index_in_collection;]],
                columns = {
                    object_name = "column_title",
                    object_artist = "column_author",
                    object_album_name = "column_binder",
                    object_album_position = "column_ord",
                    object_imprint = "column_boss",
                    object_website = "column_link",
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
                media_type = "foodoc",
                custom_detail_view = function(object_uuid, settings) end,
                play_action = function(object_uuid, settings)
                    return { action = "run", exe = "path_from_settings", args = "run=path_to_p8_file" }
                end,
                create_from_file = function(file_table, settings) end,
                import_file = function(file_path, settings) end,
                settings = {
                    store_path = { type = "filepath", default = nil }
                },
            },
        },

    }
end
