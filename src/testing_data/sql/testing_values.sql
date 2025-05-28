insert into MediaCategories
values ('DOCUMENT', 'media_category_document');
insert into MediaTypes
values ('DOCUMENT', 'plain_text_general_doc', 'DOCUMENT');
insert into MediaTypes
values ('PLAINTEXT', 'media_type_plain_text', 'DOCUMENT');
insert into MediaTypes
values ('EBOOK', 'media_type_ebook', 'DOCUMENT');
insert into MediaCategories
values ('IMAGE', 'Image');
insert into MediaTypes
values ('RASTER', 'media_type_raster_img', 'IMAGE');
insert into MediaTypes
values ('VECTOR', 'media_type_vector_img', 'IMAGE');
insert into MediaCategories
values ('AUDIO', 'Audio');
insert into MediaTypes
values ('MUSIC', 'media_type_music', 'AUDIO');
insert into MediaTypes
values ('AUDIOBOOK', 'media_type_audiobook', 'AUDIO');
insert into MediaCategories
values ('VIDEOGAME', 'media_category_videogame');
insert into MediaTypes
values ('MS', 'media_type_master_system_game', 'VIDEOGAME');
insert into MediaTypes
values ('PICO8', 'media_type_pico8_game', 'VIDEOGAME');
insert into MediaCategories
values ('ARCHIVE', 'media_category_archive');
insert into MediaTypes
values ('ARCHIVE', 'media_type_archive', 'ARCHIVE');
insert into FileExtensions
values ('TXT', 'file_ext_txt');
insert into MediaTypesForFileExtensions
values ('TXT', 'PLAINTEXT');
insert into MediaTypesForFileExtensions
values ('TXT', 'EBOOK');
insert into FileExtensions
values ('MD', 'file_ext_md');
insert into MediaTypesForFileExtensions
values ('MD', 'PLAINTEXT');
insert into MediaTypesForFileExtensions
values ('MD', 'EBOOK');
insert into FileExtensions
values ('EPUB', 'file_ext_epub');
insert into MediaTypesForFileExtensions
values ('EPUB', 'EBOOK');
insert into FileExtensions
values ('SVG', 'file_ext_svg');
insert into MediaTypesForFileExtensions
values ('SVG', 'PLAINTEXT');
insert into MediaTypesForFileExtensions
values ('SVG', 'VECTOR');
insert into FileExtensions
values ('PNG', 'file_ext_png');
insert into MediaTypesForFileExtensions
values ('PNG', 'RASTER');
insert into MediaTypesForFileExtensions
values ('PNG', 'PICO8');
insert into FileExtensions
values ('JPEG', 'file_ext_jpeg');
insert into MediaTypesForFileExtensions
values ('JPEG', 'RASTER');
insert into FileExtensions
values ('JPG', 'file_ext_jpeg');
insert into MediaTypesForFileExtensions
values ('JPG', 'RASTER');
insert into FileExtensions
values ('SMS', 'file_ext_sega_master_system');
insert into MediaTypesForFileExtensions
values ('SMS', 'MS');
insert into FileExtensions
values ('BIN', 'file_ext_bin');
insert into MediaTypesForFileExtensions
values ('BIN', 'MS');
insert into FileExtensions
values ('P8', 'file_ext_p8');
insert into MediaTypesForFileExtensions
values ('P8', 'PICO8');
insert into FileExtensions
values ('ZIP', 'file_ext_zip');
insert into MediaTypesForFileExtensions
values ('ZIP', 'ARCHIVE');
insert into MediaTypesForFileExtensions
values ('ZIP', 'MS');
insert into MediaTypesForFileExtensions
values ('ZIP', 'EBOOK');
insert into FileExtensions
values ('7Z', 'file_ext_7z');
insert into MediaTypesForFileExtensions
values ('7Z', 'ARCHIVE');
insert into MediaTypesForFileExtensions
values ('7Z', 'MS');
insert into MediaTypesForFileExtensions
values ('7Z', 'EBOOK');
insert into FileExtensions
values ('DOCX', 'file_ext_docx');
insert into MediaTypesForFileExtensions
values ('DOCX', 'ARCHIVE');
insert into MediaTypesForFileExtensions
values ('DOCX', 'DOCUMENT');
insert into FileExtensions
values ('MP3', 'file_ext_mp3');
insert into MediaTypesForFileExtensions
values ('MP3', 'MUSIC');
insert into MediaTypesForFileExtensions
values ('MP3', 'AUDIOBOOK');
insert into FileExtensions
values ('M4B', 'file_ext_m4b');
insert into MediaTypesForFileExtensions
values ('M4B', 'AUDIOBOOK');
insert into Files
values (
        'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
        "welcome.txt",
        0,
        'a38bcfa83e52932d49e36146950db40423daeb89a2e1f1b9734401bc98f1c79e',
        '',
        'TXT',
        'UTF8',
        NULL,
        FALSE,
        FALSE,
        'testingdir/alpha/'
    );
insert into FileBlobs
values (
        'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
        'Welcome to Oosikle!'
    );
insert into Files
values (
        'DEADBEEF000000000000000000000000',
        "something.png",
        0,
        '',
        'oroot://image',
        'PNG',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/alpha/'
    );
insert into Files
values (
        'DEADBEEF100000000000000000000001',
        "celeste.p8.png",
        0,
        '',
        'oroot://image',
        'PNG',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/pico8/celeste/'
    );
insert into Objects
values (
        'DEADBEEF100000000000000000000001',
        'Celeste Classic',
        'oosikle.adapter.p8',
        FALSE,
        'platformer',
        'Celeste',
        1,
        'w',
        'en',
        '@noel',
        'Maddy Makes Games',
        '1970-01-01T00:00:00',
        'https://www.lexaloffle.com/bbs/?tid=2145'
    );
insert into Files
values (
        'DEADBEEF100000000000000000000002',
        "pico_off_road.p8.png",
        0,
        '',
        'oroot://image',
        'PNG',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/pico8/'
    );
insert into Objects
values (
        'DEADBEEF100000000000000000000002',
        'Pico Off Road',
        'oosikle.adapter.p8',
        FALSE,
        'racing',
        '',
        0,
        'w',
        'en',
        '@assemblerbot',
        '',
        '1970-01-01T00:00:00',
        'https://www.lexaloffle.com/bbs/?tid=41897'
    );
insert into Files
values (
        'DEADBEEF100000000000000000000003',
        "air_delivery.p8.png",
        0,
        '',
        'oroot://image',
        'PNG',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/pico8/'
    );
insert into Objects
values (
        'DEADBEEF100000000000000000000003',
        'Air Delivery',
        'oosikle.adapter.p8',
        FALSE,
        'platformer',
        '',
        0,
        'w',
        'en',
        '@pianoman373',
        '',
        '1970-01-01T00:00:00',
        'https://www.lexaloffle.com/bbs/?tid=52598'
    );
insert into Files
values (
        'DEADBEEF100000000000000000000004',
        "celeste_classic2.p8.png",
        0,
        '',
        'oroot://image',
        'PNG',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/pico8/celeste/'
    );
insert into Objects
values (
        'DEADBEEF100000000000000000000004',
        'Celeste Classic 2',
        'oosikle.adapter.p8',
        FALSE,
        'platformer',
        'celeste',
        1,
        'w',
        'en',
        'Maddy Thorson, Noel Berry, and Lena Raine',
        'Maddy Makes Games',
        '1970-01-01T00:00:00',
        'https://www.lexaloffle.com/bbs/?tid=41282'
    );
insert into Files
values (
        'DEADBEEF100000000000000000000005',
        "hotwax.p8.png",
        0,
        '',
        'oroot://image',
        'PNG',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/pico8/'
    );
insert into Objects
values (
        'DEADBEEF100000000000000000000005',
        'Hot Wax',
        'oosikle.adapter.p8',
        FALSE,
        'puzzle',
        '',
        1,
        'w',
        'en',
        '@TRASEVOL_DOG',
        '',
        '1970-01-01T00:00:00',
        'https://www.lexaloffle.com/bbs/?pid=146729'
    );
insert into Files
values (
        'DEADBEEF100000000000000000000006',
        "picolumia-v1.2.p8.png",
        0,
        '',
        'oroot://image',
        'PNG',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/pico8/'
    );
insert into Objects
values (
        'DEADBEEF100000000000000000000006',
        'Picolumia v1.2',
        'oosikle.adapter.p8',
        FALSE,
        'puzzle',
        '',
        1,
        'w',
        'en',
        '@andrewedstrom',
        '',
        '1970-01-01T00:00:00',
        'https://www.lexaloffle.com/bbs/?tid=39935'
    );
insert into Files
values (
        'DEADBEEF000000000000000000000002',
        "zzzsplore.p8",
        0,
        '',
        'oroot://game',
        'P8',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/pico8/'
    );
insert into Files
values (
        'DEADBEEF000000000000000000000003',
        "ultimate_soccer.sms",
        0,
        '',
        'oroot://game',
        'SMS',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/mastersystem/'
    );
insert into Files
values (
        'DEADBEEF000000000000000000000004',
        "aladdin.zip",
        0,
        '',
        'oroot://archive',
        'ZIP',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/mastersystem/'
    );
insert into Files
values (
        'DEADBEEF000000000000000000000005',
        "sample1.mp3",
        0,
        '',
        'oroot://audio',
        'MP3',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/beta/'
    );
insert into Files
values (
        'DEADBEEF000000000000000000000006',
        "sample2.mp3",
        0,
        '',
        'oroot://audio',
        'MP3',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/beta/'
    );
insert into Files
values (
        'DEADBEEF000000000000000000000007',
        "abook1.m4b",
        0,
        '',
        'oroot://audio',
        'M4B',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/gamma/'
    );
insert into Files
values (
        'DEADBEEF000000000000000000000008',
        "there_and_back_again.epub",
        0,
        '',
        'oroot://document',
        'EPUB',
        'BIN',
        NULL,
        FALSE,
        FALSE,
        'testingdir/gamma/'
    );
insert into Files
values (
        'DEADBEEF000000000000000000000009',
        "intrepreterbook.md",
        0,
        '',
        'oroot://document',
        'MD',
        'UTF8',
        NULL,
        FALSE,
        FALSE,
        'testingdir/booki/'
    );
insert into Objects
values (
        'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
        'Welcome File',
        'oosikle.adapter.text',
        FALSE,
        '',
        '',
        0,
        'w',
        'en',
        'TheHotFish',
        '',
        '1970-01-01T00:00:00',
        ''
    );
insert into ObjectAttributes
values (
        'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
        'revision',
        4
    ),
    (
        'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
        'rating',
        3.9
    ),
    (
        'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
        'editable',
        TRUE
    );
insert into Collections
values (
        'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
        'Default Briefcase',
        TRUE,
        '',
        FALSE
    );
insert into MediaCategoriesForCollections
values (
        'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
        'DOCUMENT'
    );
insert into ObjectsInCollections
values (
        'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
        0,
        'DEADBEEFDEADBEEFDEADBEEFDEADBEEF'
    );
insert into Collections
values (
        'BADBEEF7DEADBEEF4242424242424242',
        'Pico 8 Favorites',
        TRUE,
        '',
        FALSE
    );
insert into MediaCategoriesForCollections
values (
        'BADBEEF7DEADBEEF4242424242424242',
        'videogame'
    );
insert into ObjectsInCollections
values (
        'BADBEEF7DEADBEEF4242424242424242',
        0,
        'DEADBEEF100000000000000000000001'
    ),
    (
        'BADBEEF7DEADBEEF4242424242424242',
        1,
        'DEADBEEF100000000000000000000003'
    ),
    (
        'BADBEEF7DEADBEEF4242424242424242',
        3,
        'DEADBEEF100000000000000000000002'
    ),
    (
        'BADBEEF7DEADBEEF4242424242424242',
        2,
        'DEADBEEF100000000000000000000004'
    ),
    (
        'BADBEEF7DEADBEEF4242424242424242',
        4,
        'DEADBEEF100000000000000000000006'
    );
insert into Devices
values (
        '0DE2C3400DE2C3400DE2C3400DE2C340',
        'Example Flash Drive',
        'This is a flash drive that is used to test the program',
        NULL
    );
insert into DeviceSyncLists
values (
        '0DE2C3400DE2C3400DE2C3400DE2C340',
        'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
        'oosikle.adapter.text',
        'documents',
        0
    );
