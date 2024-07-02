create table files {
    id  int,
    name varchar(120)
}

create table lines {
    id int,
    file_id int references files(id)
}

create table words {
    id int,
    file_id int references lines(id),
    word varchar(120)
}
