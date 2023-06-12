-- Create most basic database example
create table if not exists authors (
    id bigint primary key generated always as identity,
    name text not null,
    display_name text not null,
    created timestamp with time zone not null default now()
);

create table if not exists books (
    id bigint primary key generated always as identity,
    name text unique not null,
    description text
);

create table if not exists authors_of_books (
    id bigint primary key generated always as identity,
    author_id bigint not null,
    book_id bigint not null,
    foreign key (author_id) references authors (id),
    foreign key (book_id) references books (id)
);

-- Create some testdata
insert into authors (name, display_name)
    values 
    ('muster', 'Max Mustermann'),
    ('test', 'Tami Test'),
    ('rswan', 'Ron Swanson');

insert into books (name, description)
    values 
    ('Flies of the Dead', 'Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam'),
    ('To Capture a Ring', 'Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.'),
    ('Love and Leather', 'Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum.');

insert into authors_of_books (author_id, book_id)
    values
    (1, 1),
    (1, 2),
    (1, 3),
    (2, 2),
    (2, 3),
    (3, 3);
