insert into MediaCategories values ('DOCUMENT', 'Document');

insert into MediaTypes values ('PLAINTEXT', 'Plain Text File', 'DOCUMENT');

insert into FileExtensions values ('TXT', 'Ordinary text file');

insert into MediaTypesForFileExtensions values ('TXT', 'PLAINTEXT');

insert into Files values (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    "welcome.txt",
    0,
    'a38bcfa83e52932d49e36146950db40423daeb89a2e1f1b9734401bc98f1c79e',
    '',
    'TXT',
    'UTF-8',
    NULL,
    FALSE,
    FALSE
);

insert into FileBlobs values (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'Welcome to Oosikle!'
);

insert into Plugins values (
    'oosikle.manager.text',
    'Default Text File Manager',
    1
);

insert into MediaTypesForPlugins values (
    'oosikle.manager.text',
    'PLAINTEXT'
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
);

insert into ObjectAttributes values (
    'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'rating',
    3.9
);

insert into ObjectAttributes values (
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
