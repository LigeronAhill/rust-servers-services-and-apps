/* drop tables if they already exist*/
DROP TABLE IF EXISTS ezy_course_c6 CASCADE;
DROP TABLE IF EXISTS ezy_tutor_c6;
/* create tables. */
CREATE TABLE ezy_tutor_c6 (
    tutor_id SERIAL PRIMARY KEY,
    tutor_name VARCHAR(200) NOT NULL,
    tutor_pic_url VARCHAR(200) NOT NULL,
    tutor_profile VARCHAR(2000) NOT NULL
);
CREATE TABLE ezy_course_c6 (
    course_id SERIAL PRIMARY KEY,
    tutor_id INT NOT NULL,
    course_name VARCHAR(140) NOT NULL,
    course_description VARCHAR(2000),
    course_format VARCHAR(30),
    course_structure VARCHAR(200),
    course_duration VARCHAR(30),
    course_price INT,
    course_language VARCHAR(30),
    course_level VARCHAR(30),
    posted_time TIMESTAMP DEFAULT now(),
    CONSTRAINT fk_tutor FOREIGN KEY(tutor_id) REFERENCES ezy_tutor_c6(tutor_id) ON DELETE CASCADE
);
GRANT all privileges ON TABLE ezy_tutor_c6 to truuser;
/* load seed data for testing */
insert into ezy_tutor_c6(
        tutor_name,
        tutor_pic_url,
        tutor_profile
    )
values(
        'merlene',
        'http://s3.amazon.aws.com/pic1',
        'merlene is an experienced finance professional'
    );
insert into ezy_tutor_c6(
        tutor_name,
        tutor_pic_url,
        tutor_profile
    )
values(
        'frank',
        'http://s3.amazon.aws.com/pic2',
        'frank is an expert nuclear engineer'
    );
insert into ezy_course_c6 (
        tutor_id,
        course_name,
        course_level,
        posted_time
    )
values(
        1,
        'first course',
        'beginner',
        '2021-04-12 05:40:00'
    );
insert into ezy_course_c6 (
        tutor_id,
        course_name,
        course_format,
        posted_time
    )
values(
        1,
        'second course',
        'ebook',
        '2021-04-12 05:45:00'
    );