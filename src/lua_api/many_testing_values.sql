insert into MediaCategories values ('DOCUMENT', 'Document');
insert into MediaTypes values ('DOCUMENT', 'General Document', 'DOCUMENT');
insert into MediaTypes values ('PLAINTEXT', 'Plain Text File', 'DOCUMENT');
insert into MediaTypes values ('EBOOK', 'Electronic Book file', 'DOCUMENT');

insert into MediaCategories values ('IMAGE', 'Image');
insert into MediaTypes values ('RASTER', 'Raster Image File', 'IMAGE');
insert into MediaTypes values ('VECTOR', 'Vector Image File', 'IMAGE');

insert into MediaCategories values ('AUDIO', 'Audio');
insert into MediaTypes values ('MUSIC', 'Music File', 'AUDIO');
insert into MediaTypes values ('AUDIOBOOK', 'Audio book file', 'AUDIO');

insert into MediaCategories values ('VIDEOGAME', 'Video Game');
insert into MediaTypes values ('MS', 'Master System Game', 'VIDEOGAME');
insert into MediaTypes values ('PICO8', 'Pico8 Game', 'VIDEOGAME');

insert into MediaCategories values ('ARCHIVE', 'Archive of one or more files');
insert into MediaTypes values ('ARCHIVE', 'Compressed Archive', 'ARCHIVE');

insert into FileExtensions values ('TXT', 'Ordinary text file');
insert into MediaTypesForFileExtensions values ('TXT', 'PLAINTEXT');
insert into MediaTypesForFileExtensions values ('TXT', 'EBOOK');

insert into FileExtensions values ('MD', 'Markdown text file');
insert into MediaTypesForFileExtensions values ('MD', 'PLAINTEXT');
insert into MediaTypesForFileExtensions values ('MD', 'EBOOK');

insert into FileExtensions values ('EPUB', 'Epub book file');
insert into MediaTypesForFileExtensions values ('EPUB', 'EBOOK');

insert into FileExtensions values ('SVG', 'Scalable Vector Graphics file');
insert into MediaTypesForFileExtensions values ('SVG', 'PLAINTEXT');
insert into MediaTypesForFileExtensions values ('SVG', 'VECTOR');

insert into FileExtensions values ('PNG', 'Portable Network Graphics file');
insert into MediaTypesForFileExtensions values ('PNG', 'RASTER');
insert into MediaTypesForFileExtensions values ('PNG', 'PICO8');

insert into FileExtensions values ('JPEG', 'JPEG Image File');
insert into MediaTypesForFileExtensions values ('JPEG', 'RASTER');
insert into FileExtensions values ('JPG', 'JPEG Image File');
insert into MediaTypesForFileExtensions values ('JPG', 'RASTER');

insert into FileExtensions values ('SMS', 'Master System Game ROM');
insert into MediaTypesForFileExtensions values ('SMS', 'MS');

insert into FileExtensions values ('BIN', 'Binary file');
insert into MediaTypesForFileExtensions values ('BIN', 'MS');

insert into FileExtensions values ('P8', 'Pico 8 File');
insert into MediaTypesForFileExtensions values ('P8', 'PICO8');

insert into FileExtensions values ('ZIP', 'Zip File');
insert into MediaTypesForFileExtensions values ('ZIP', 'ARCHIVE');
insert into MediaTypesForFileExtensions values ('ZIP', 'MS');
insert into MediaTypesForFileExtensions values ('ZIP', 'EBOOK');

insert into FileExtensions values ('7Z', '7Zip File');
insert into MediaTypesForFileExtensions values ('7Z', 'ARCHIVE');
insert into MediaTypesForFileExtensions values ('7Z', 'MS');
insert into MediaTypesForFileExtensions values ('7Z', 'EBOOK');

insert into FileExtensions values ('DOCX', 'Microsoft Word Document File');
insert into MediaTypesForFileExtensions values ('DOCX', 'ARCHIVE');
insert into MediaTypesForFileExtensions values ('DOCX', 'DOCUMENT');

insert into FileExtensions values ('MP3', 'MPEG-3 Audio File');
insert into MediaTypesForFileExtensions values ('MP3', 'MUSIC');
insert into MediaTypesForFileExtensions values ('MP3', 'AUDIOBOOK');

insert into FileExtensions values ('M4B', 'MPEG-3 Audiobook File');
insert into MediaTypesForFileExtensions values ('M4B', 'AUDIOBOOK');

insert into Plugins values (
    'oosikle.manager.text',
    'Default Text File Manager',
    1
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.text',
    'PLAINTEXT'
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.text',
    'DOCUMENT'
);

insert into Plugins values (
    'oosikle.manager.books',
    'Default Book Manager',
    1
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.books',
    'EBOOK'
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.books',
    'AUDIOBOOK'
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.books',
    'DOCUMENT'
);

insert into Plugins values (
    'oosikle.manager.images',
    'Default Images Manager',
    1
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.images',
    'RASTER'
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.images',
    'VECTOR'
);

insert into Plugins values (
    'oosikle.manager.roms',
    'Default Games Manager',
    1
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.roms',
    'MS'
);

insert into Plugins values (
    'oosikle.manager.p8',
    'Default Pico 8 Manager',
    1
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.p8',
    'PICO8'
);

insert into Plugins values (
    'oosikle.manager.music',
    'Default Music Manager',
    1
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.music',
    'MUSIC'
);

insert into Files values (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    "welcome.txt",
    0,
    'a38bcfa83e52932d49e36146950db40423daeb89a2e1f1b9734401bc98f1c79e',
    '',
    'TXT',
    'UTF8',
    NULL,
    FALSE,
    FALSE
);

insert into FileBlobs values (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'Welcome to Oosikle!'
);

insert into Files values (
    'DEADBEEF000000000000000000000000',
    "something.png",
    0,
    '',
    'oroot://image',
    'PNG',
    'BIN',
    NULL,
    FALSE,
    FALSE
);

insert into Files values (
    'DEADBEEF000000000000000000000001',
    "celeste.p8.png",
    0,
    '',
    'oroot://image',
    'PNG',
    'BIN',
    NULL,
    FALSE,
    FALSE
);

insert into Files values (
    'DEADBEEF000000000000000000000002',
    "zzzsplore.p8",
    0,
    '',
    'oroot://game',
    'P8',
    'BIN',
    NULL,
    FALSE,
    FALSE
);

insert into Files values (
    'DEADBEEF000000000000000000000003',
    "ultimate_soccer.sms",
    0,
    '',
    'oroot://game',
    'SMS',
    'BIN',
    NULL,
    FALSE,
    FALSE
);

insert into Files values (
    'DEADBEEF000000000000000000000004',
    "aladdin.zip",
    0,
    '',
    'oroot://archive',
    'ZIP',
    'BIN',
    NULL,
    FALSE,
    FALSE
);

insert into Files values (
    'DEADBEEF000000000000000000000005',
    "sample1.mp3",
    0,
    '',
    'oroot://audio',
    'MP3',
    'BIN',
    NULL,
    FALSE,
    FALSE
);

insert into Files values (
    'DEADBEEF000000000000000000000006',
    "sample2.mp3",
    0,
    '',
    'oroot://audio',
    'MP3',
    'BIN',
    NULL,
    FALSE,
    FALSE
);

insert into Files values (
    'DEADBEEF000000000000000000000007',
    "abook1.m4b",
    0,
    '',
    'oroot://audio',
    'M4B',
    'BIN',
    NULL,
    FALSE,
    FALSE
);

insert into Files values (
    'DEADBEEF000000000000000000000008',
    "there_and_back_again.epub",
    0,
    '',
    'oroot://document',
    'EPUB',
    'BIN',
    NULL,
    FALSE,
    FALSE
);

insert into Files values (
    'DEADBEEF000000000000000000000009',
    "intrepreterbook.md",
    0,
    '',
    'oroot://document',
    'MD',
    'UTF8',
    NULL,
    FALSE,
    FALSE
);

insert into Objects values (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'Welcome File',
    'oosikle.manager.text',
    FALSE
);

insert into ObjectAttributes values (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'author',
    'TheHotFish'
);

insert into ObjectAttributes values (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'revision',
    4
), (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'rating',
    3.9
), (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'editable',
    TRUE
);

insert into Collections values (
    'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    'Default Briefcase',
    TRUE,
    '',
    FALSE
);

insert into MediaCategoriesForCollections values (
    'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    'DOCUMENT'
);

insert into ObjectsInCollections values (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    0
);

insert into Devices values (
    '0DE2C3400DE2C3400DE2C3400DE2C340',
    'Example Flash Drive',
    'This is a flash drive that is used to test the program',
    NULL
);

insert into DeviceSyncLists values (
    '0DE2C3400DE2C3400DE2C3400DE2C340',
    'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    'oosikle.manager.text',
    'documents',
    0
);
