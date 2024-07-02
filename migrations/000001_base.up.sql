create table files (
    id  int not null primary key,
    name varchar(120)
);

create table lines (
    id int not null primary key,
    file_id int references files(id)
);

create table words (
    id int not null primary key,
    file_id int references lines(id),
    word varchar(120)
);
