CREATE TABLE students (
    username varchar(100) NOT NULL PRIMARY KEY,
    salt varchar(100),
    pw_hash varchar(200),
    full_name varchar(200),
    start_date varchar(30),
    status varchar(20),
    address varchar(400),
    email varchar(100) NOT NULL UNIQUE,
    mobile varchar(30) NOT NULL UNIQUE
);

CREATE TABLE instructors (
    username varchar(100) NOT NULL PRIMARY KEY,
    salt varchar(100),
    pw_hash varchar(200),
    full_name varchar(200),
    about varchar(4000),
    total_students INT,
    tags varchar(400),
    start_date varchar(30),
    status varchar(20),
    address varchar(400),
    email varchar(100) NOT NULL UNIQUE,
    mobile varchar(30) NOT NULL UNIQUE
);

CREATE TABLE courses (
    course_id char(36) NOT NULL PRIMARY KEY,
    title varchar(400),
    price FLOAT,
    course_language varchar(50),
    rating INT,
    target_level varchar(10),
    requirement varchar(400),
    duration_minutes INT,
    about varchar(500),
    description varchar(500),
    tag_line varchar(300),
    update_date varchar(40),
    status varchar(100),
    series_id char(36)
);

CREATE TABLE student_course (
    username varchar(100) NOT NULL,
    course_id char(36) NOT NULL,
    allow_code bool,
    priority INT,
    PRIMARY KEY(username, course_id)
);

CREATE TABLE course_instructor (
    course_id char(36) NOT NULL,
    username varchar(100) NOT NULL,
    full_name varchar(200),
    PRIMARY KEY(course_id, username)
);

CREATE TABLE chapters (
    chapter_id char(36) NOT NULL PRIMARY KEY,
    title varchar(400),
    content TEXT,
    chapter_number INT,
    course_id char(36)
);

CREATE TABLE tutorials (
	tutorial_id char(36) NOT NULL PRIMARY KEY,
	course_id char(36),
	chapter_number INT,
	code_content varchar(3000)
);

CREATE TABLE series (
    series_id char(36) NOT NULL PRIMARY KEY,
    title varchar(400),
    price FLOAT,
    course_language varchar(50),
    rating INT,
    target_level varchar(10),
    requirement varchar(400),
    duration_minutes INT,
    about varchar(500),
    description varchar(500),
    tag_line varchar(300),
    update_date varchar(40),
    status varchar(100)
);

CREATE TABLE student_series (
    username varchar(100) NOT NULL,
    series_id char(36) NOT NULL,
    PRIMARY KEY(username, series_id)
);

INSERT INTO students (username, salt, pw_hash, start_date, full_name, status, email, mobile)
VALUES ('student1', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '2024-0821', '学生 1', 'active', 'student1@example.com', '18602341234');

INSERT INTO students (username, salt, pw_hash, start_date, full_name, status, email, mobile)
VALUES ('student2', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '2024-0821', '学生 2', 'active', 'student2@example.com', '18602341235');

INSERT INTO students (username, salt, pw_hash, start_date, full_name, status, email, mobile)
VALUES ('student3', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '2024-0821', '学生 3', 'active', 'student3@example.com', '18602341236');

INSERT INTO instructors (username, salt, pw_hash, start_date, full_name, about, status, email, mobile)
VALUES ('teacher1', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '2024-0821', '教师 1', '10 年教学经验', 'active',  'teacher1@example.com', '18602341237');

INSERT INTO courses (course_id, title, price, course_language, rating, target_level, requirement, duration_minutes, about, description, tag_line, update_date, status, series_id)
VALUES ('97931561-7689-44a4-bf80-f2e7c9e8d2dd', 'Python - 面向物联网控制', 100.00, '中文', 9, '初学者', '无需经验', 960, '对Python语言有基本的了解.对物联网有基本的了解.','面对小学生和初中生的Python编成教学课程。以物联网为背景。','以物联网为背景,面对小学生和初中生的Python编成教学课程。', '2024-08-20', 'live', 'series-01');

INSERT INTO courses (course_id, title, price, course_language, rating, target_level, requirement, duration_minutes, about, description, tag_line, update_date, status, series_id)
VALUES ('10031561-7689-44a4-bf80-f2e7c9e8d2dd', 'C 编程语言基础', 100.00, '中文', 9, '初学者', '无需经验', 960, '对C语言有基本的了解.','面对所有人的C编成教学课程。','基础C语言编成教学课程。', '2024-08-21', 'live', 'series-01');

INSERT INTO student_course (username, course_id, allow_code, priority)
VALUES ('student1', '97931561-7689-44a4-bf80-f2e7c9e8d2dd', true, 1);

INSERT INTO student_course (username, course_id, allow_code, priority)
VALUES ('student1', '10031561-7689-44a4-bf80-f2e7c9e8d2dd', true, 2);

INSERT INTO student_course (username, course_id, allow_code, priority)
VALUES ('student2', '10031561-7689-44a4-bf80-f2e7c9e8d2dd', true, 1);

INSERT INTO student_course (username, course_id, allow_code, priority)
VALUES ('student3', '97931561-7689-44a4-bf80-f2e7c9e8d2dd', true, 1);

INSERT INTO student_course (username, course_id, allow_code, priority)
VALUES ('student3', '10031561-7689-44a4-bf80-f2e7c9e8d2dd', true, 2);

INSERT INTO course_instructor (course_id, username)
VALUES ('97931561-7689-44a4-bf80-f2e7c9e8d2dd', 'teacher1');

INSERT INTO chapters (chapter_id, title, content, chapter_number, course_id)
VALUES ('welcome-0000', '欢迎', '# 欢迎来到课程', 0, '00000000-0000-0000-0000-000000000000');

INSERT INTO chapters (chapter_id, title, content, chapter_number, course_id)
VALUES ('python-0000-001', '介绍', '# 介绍', 1, '97931561-7689-44a4-bf80-f2e7c9e8d2dd');

INSERT INTO chapters (chapter_id, title, content, chapter_number, course_id)
VALUES ('python-0000-002', '背景', '# 背景', 2, '97931561-7689-44a4-bf80-f2e7c9e8d2dd');

INSERT INTO chapters (chapter_id, title, content, chapter_number, course_id)
VALUES ('c-0000-001', '介绍', '# 介绍', 1, '10031561-7689-44a4-bf80-f2e7c9e8d2dd');

INSERT INTO chapters (chapter_id, title, content, chapter_number, course_id)
VALUES ('c-0000-002', '背景', '# 背景', 2, '10031561-7689-44a4-bf80-f2e7c9e8d2dd');

INSERT INTO tutorials (tutorial_id, chapter_number, code_content, course_id)
VALUES ('t-python-0000-001', 1, 'print("Hello, 1")', 'python-0000-001');

INSERT INTO tutorials (tutorial_id, chapter_number, code_content, course_id)
VALUES ('t-python-0000-002', 2, 'print("Hello, 2")', 'python-0000-002');

INSERT INTO tutorials (tutorial_id, chapter_number, code_content, course_id)
VALUES ('t-c-0000-001', 1, '#include <stdio.h>', 'c-0000-001');

INSERT INTO tutorials (tutorial_id, chapter_number, code_content, course_id)
VALUES ('t-c-0000-002', 2, '#include <stdio.h>
int main()', 'c-0000-002');
