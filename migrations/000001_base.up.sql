create table sources (
    id  int not null primary key,
    url text not null
);

create table lines (
    id int not null primary key,
    source_id int references sources(id)
);

create table words (
    id int not null primary key,
    source_id int references lines(id),
    word varchar(120)
);
