insert into MediaCategories values ('DOCUMENT', 'Document');

insert into MediaTypes values ('PLAINTEXT', 'Plain Text File', 'DOCUMENT');

insert into FileExtensions values ('TXT', 'Ordinary text file');

insert into MediaTypesForFileExtensions values ('TXT', 'PLAINTEXT');

insert into Files values (
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
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
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
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
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'Welcome File',
    'oosikle.manager.text',
    FALSE
);

insert into ObjectAttributes values (
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'author',
    'TheHotFish'
);

insert into ObjectAttributes values (
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'revision',
    4
);

insert into ObjectAttributes values (
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'rating',
    3.9
);

insert into ObjectAttributes values (
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    'editable',
    TRUE
);

insert into Collections values (
    X'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    'Default Briefcase',
    TRUE,
    '',
    FALSE
);

insert into MediaCategoriesForCollections values (
    X'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    'DOCUMENT'
);

insert into ObjectsInCollections values (
    X'DEADBEEFDEADBEEFDEADBEEFDEADBEEF',
    X'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    0
);

insert into Devices values (
    X'0DE2C3400DE2C3400DE2C3400DE2C340',
    'Example Flash Drive',
    'This is a flash drive that is used to test the program',
    NULL
);

insert into DeviceSyncLists values (
    X'0DE2C3400DE2C3400DE2C3400DE2C340',
    X'BADC0FFEE0DDF00DBADC0FFEE0DDF00D',
    'oosikle.manager.text',
    'documents',
    0
);
