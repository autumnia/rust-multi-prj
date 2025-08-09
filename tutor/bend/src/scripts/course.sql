------------------------------------------------------------
-- [version 1]
------------------------------------------------------------
-- create user tutor with password '0823';
-- grant all privileges on database postgres to tutor;
-- ALTER role tutor CREATEDB ;

-- 테이블이 존재하면 삭제한다
drop table if exists course;
-- 테이블을 생성한다.
create table course
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

-- 테스팅을 위한 시드 데이터를 로드한다
insert into course(course_id,tutor_id, course_name,posted_time) values(1, 1, 'First course', '2020-12-17 05:40:00');
insert into course(course_id, tutor_id, course_name,posted_time) values(2, 1, 'Second course', '2020-12-18 05:45:00');


------------------------------------------------------------
-- [version 2]
------------------------------------------------------------
drop table if exists course;

create table course
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    course_description varchar(2000),
    course_format varchar(30),
    course_structure varchar(200),
    course_duration varchar(30),
    course_price INT,
    course_language varchar(30),
    course_level varchar(30),
    posted_time TIMESTAMP default now()
);

create user postgres with password '0823';
grant all privileges on table course to postgres;
grant all privileges on all sequences in schema public to postgres;