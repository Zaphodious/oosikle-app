Plugin:Credit({
    authors = { "HotFish", "Croug" },
    version = 1,
    date = "2025-05-04"
})

Plugin:DefMediaCategory {
    name = "videogame",
    types = {
        { name = "pico8",
            file_extensions = { "p8", "png" } }
    },
}

Plugin:DefFileExtension {
    tag = "p8.png",
    type = "pico8"
}

Plugin:DefViewAdapter {
    -- could be media_category or media_type
    media_category = "videogame",
    -- there will be a default sql query, but if a view really really needs to do something special there's the option.
    -- This can be either a string or a function accepting settings as its only param.
    page_sql = [[select * from objects where objects.collection_uuid=:coll_uuid]],
    -- there will be default views that can be used for any collection of objects, and if any custom names
    -- are required they can be provided here (instead of needing to define bespoke view logic). This table
    -- will be logically merged with a default table before translation that has a bare minimum of columns defined.
    -- Only columns that have an entry in the final mapping table will be given a column using the default views
    -- Note that this table is also provided to custom views, to avoid duplication.
    default_columns = {
        object_name = "title",
        object_artist = "developer",
        object_album_name = "series",
        object_album_position = "series_entry",
        object_imprint = "publisher",
        file_size_bytes = { name = "file_size", formatter = "format_string" },
    },
    -- For the default view
    default_assemble_object = function(raw_object_table, settings)
        -- Takes in a row table from page_sql and spits out a table with the expected fields.
        -- Optional. If doesn't exist, the row will just be passed into the
        -- view function as-is.
    end,
    -- If custom views are desired, they can be defined here
    views = {
        {
            -- This is what the user will see if the theme shows the text of the option
            viewtype = "table",
            -- This is responsible for setting up the HTML container and injecting the elements into it
            initial_view_fn = function(list_of_object_tables, column_mappings, offset, settings)
                -- just imagine an html-returning function here
            end,
            -- This is responsible for taking in the next page of elements and rendering them. Will be called
            -- when the front-end gets an event signaling that it needs another page. If this does not exist,
            -- then initial_view_fn will be called instead. Think infinite scroll vs discretly numbered pages
            update_view_fn = function(list_of_object_tables, column_mappings, offset, settings)
                -- imagine another one here
            end,
        },
        {
            viewtype = "card",
            initial_view_fn = function(list_of_object_tables, column_mappings, settings) end,
            update_view_fn = function(list_of_object_tables, column_mappings, settings) end,
            -- If a view requires its own specific assembly function,
            -- it can be defined here.
            assemble_object = function(raw_object_table, column_mappings, settings) end,
        }
    },
    -- If this is present, then there will be an option to "play" a collection
    -- being displayed by this view adapter. For example, a collection of
    -- media_category:audio can be opened in VLC, or a collection of
    -- media_category:image can be opened as a gallery.
    play_action = function() end,
    -- A table defining the adapter's settings. Changed values are stored in the database.
    -- Callbacks in this adapter will be provided with a table composed of the defaults
    -- provided here overridden with any changed values.
    settings = {
        skraper_api_key = { type = "string", default = nil }
    },
}

-- For handling individual objects, an adapter can be registered to ensure that
-- the special needs of the object are taken care of
Plugin:DefObjectAdapter {
    -- Used to determine not only an assigned object's type,
    -- but so that the app knows which object adapters to
    -- offer to be a givne object's handler. For example, if a
    -- user adds a .png file to the app, the system could
    -- offer either this pico8 adapter or the default image adapter
    media_type = "pico8",
    -- There will be a "detail" view, that by default shows
    -- all of the information about an object in a fairly
    -- straightforward way. If a custom detail view is desired,
    -- this is how to provide the HTML for it. Note that it is possible
    -- for the default view to not be offered to the user,
    -- depending on the view being used to display the current
    -- collection.
    custom_detail_view = function(object_uuid, settings) end,
    -- Similar to the one in the view adapter, if this is present
    -- then there will be the option to "play" objects using
    -- this object adapter. For example, this example pico8 plugin
    -- could use the path to the pico8 executable to load a cart, or
    -- a plugin for an ebook could use an ebook app registered with it,
    -- or a plugin for 3D print files could load a web view with a 3D
    -- renderer. If the play action for a collection is defined and
    -- the user decides to play the collection, then that will be used
    -- and this will not be called.
    play_action = function(object_uuid, settings)
        return { action = "run", exe = "path_from_settings", args = "run=path_to_p8_file" }
    end,
    -- If a file is intended to be used as an object, this function will be called.
    -- This function is responsible for storing the object's data in the db,
    -- storing any necessary extended attributes, and associating any additional files
    -- with the object (example- album art, .sav files for ROMs, etc). If this is happening
    -- during an initial import, the file_table will have its path set to the location
    -- of the original file, and this function will be able to ask for more information
    -- about other files in that context.
    initialize_object = function(file_table, settings) end,
    -- A table defining the adapter's settings. Changed values are stored in the database.
    -- Callbacks in this adapter will be provided with a table composed of the defaults
    -- provided here overridden with any changed values.
    settings = {
        pico8path = { type = "filepath", default = nil }
    },
}

-- Extensions are bundles of additional functionality, and can add new features to the app.
Plugin:DefExtension {
    -- While the package name for adapters is computed automatically, each extension
    -- must provide its own, which is appended to the fully qualified package name of
    -- its containing plugin.
    name = "importers.retroarchlib",
    -- lifecycle functions. Unsure how this will work exactly, but should probably have this?
    on_init = function() end,
    on_teardown = function() end, -- dubious if this is possible?
    -- Any actions listed here will be displayed to the user, and will be ran when the user selects
    -- them from the menu
    actions = {
        import_retroarch_library = function(settings)
            --[[ example psudolua
            local retroarch_path = App:PromptForPath()
            local rom_path = retroarch_path + path("roms")
            local save_path = retroarch_path + path("save")
            local progress = App:StartProgress("retroarch_import")
            for file in glob(rom_path) do
                progress:update("retroarch_import", index/total)
                local save_file_path = find_save(save_path, file)
                App:ImportFile {filepath = file, extrafiles = {save_file_path,}}
            end
            progress:end()
            --]]
        end,
    }
}
