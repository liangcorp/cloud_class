CREATE TABLE sponsors (
    uuid varchar(100) NOT NULL PRIMARY KEY,
    org_name varchar(200),
    priority INT
);

CREATE TABLE administrators (
    username varchar(100) NOT NULL PRIMARY KEY,
    salt varchar(100),
    pw_hash varchar(200)
);

CREATE TABLE students (
    username varchar(100) NOT NULL PRIMARY KEY,
    salt varchar(100),
    pw_hash varchar(200),
    fullname varchar(200),
    start_date varchar(30),
    status varchar(20),
    address varchar(400),
    email varchar(100) NOT NULL UNIQUE,
    mobile varchar(30) NOT NULL UNIQUE,
    container_port INT CHECK (container_port >= 8500 AND container_port <= 65500),
    profile_image_id varchar(100) UNIQUE
);

CREATE TABLE instructors (
    username varchar(100) NOT NULL PRIMARY KEY,
    salt varchar(100),
    pw_hash varchar(200),
    fullname varchar(200),
    about varchar(4000),
    total_students INT,
    tag_line varchar(400),
    start_date varchar(30),
    status varchar(20),
    address varchar(400),
    email varchar(100) NOT NULL UNIQUE,
    mobile varchar(30) NOT NULL UNIQUE,
    priority INT,
    rating INT,
    profile_image_id varchar(100) UNIQUE
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
    series_id char(36),
    image_id varchar(100) UNIQUE
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
    fullname varchar(200),
    priority INT,
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
	chapter_number INT,
	code_content varchar(3000),
	chapter_id char(36),
	course_id char(36)
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

INSERT INTO students (username, salt, pw_hash, start_date, fullname, status, email, mobile, container_port)
VALUES ('student1', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '2024-08-21', '学生 1', 'active', 'student1@example.com', '18602341234', 8501);

INSERT INTO students (username, salt, pw_hash, start_date, fullname, status, email, mobile, container_port)
VALUES ('student2', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '2024-08-21', '学生 2', 'active', 'student2@example.com', '18602341235', 8502);

INSERT INTO students (username, salt, pw_hash, start_date, fullname, status, email, mobile, container_port)
VALUES ('student3', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '2024-08-21', '学生 3', 'active', 'student3@example.com', '18602341236', 8503);

INSERT INTO students (username, salt, pw_hash, start_date, fullname, status, email, mobile, container_port)
VALUES ('student4', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '2024-08-21', '学生 4', 'active', 'student4@example.com', '18602341237', (SELECT container_port FROM students ORDER BY container_port DESC LIMIT 1) + 1);

INSERT INTO instructors (username, salt, pw_hash, fullname, about, total_students, tag_line, start_date, status, email, mobile, priority, rating, profile_image_id)
VALUES ('teacher1', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '教师 1', '10年Python教学经验', 100, '10年Python教学经验', '2024-08-21', 'active', 'teacher1@example.com', '18602341237', 1, 5, '0c2a3ed6fc1d62a5ff36a99cbdead670.jpeg');

INSERT INTO instructors (username, salt, pw_hash, fullname, about, total_students, tag_line, start_date, status, email, mobile, priority, rating, profile_image_id)
VALUES ('teacher2', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8', '教师 2', '10年C教学经验', 100, '10年C教学经验', '2024-08-21', 'active', 'teacher2@example.com', '18602341238', 2, 4, 'bda9ff00a6db34f77844bf8718bc0832.webp');

INSERT INTO administrators (username, salt, pw_hash)
VALUES ('admin1', 'x1z2S4jDbLrigzigZp9CdA', 'zhZt3RLLVZV9watjOg/gIvAhjuOvSox9JOf2nxdZ2S8');

INSERT INTO courses (course_id, title, price, course_language, rating, target_level, requirement, duration_minutes, about, description, tag_line, update_date, status, series_id, image_id)
VALUES ('97931561-7689-44a4-bf80-f2e7c9e8d2dd', 'Python - 面向物联网控制', 100.00, '中文', 9, '初学者', '无需经验', 960, '对Python语言有基本的了解.对物联网有基本的了解.','面对小学生和初中生的Python编成教学课程。以物联网为背景。','以物联网为背景,面对小学生和初中生的Python编成教学课程。', '2024-08-20', 'live', 'series-01', '75a3bef8ec10201d474dedb27eeaf3d5.webp');

INSERT INTO courses (course_id, title, price, course_language, rating, target_level, requirement, duration_minutes, about, description, tag_line, update_date, status, series_id, image_id)
VALUES ('10031561-7689-44a4-bf80-f2e7c9e8d2dd', 'C 编程语言基础', 100.00, '中文', 9, '初学者', '无需经验', 960, '对C语言有基本的了解.','面对所有人的C编成教学课程。','基础C语言编成教学课程。', '2024-08-21', 'live', 'series-01', '091c0066136730598dd01a0cfeb2d6c6.png');

INSERT INTO course_instructor (course_id, username, fullname, priority)
VALUES ('97931561-7689-44a4-bf80-f2e7c9e8d2dd', 'teacher1', '教师 1', 1);

INSERT INTO course_instructor (course_id, username, fullname, priority)
VALUES ('10031561-7689-44a4-bf80-f2e7c9e8d2dd', 'teacher2', '教师 2', 1);

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

INSERT INTO tutorials (tutorial_id, chapter_number, code_content, chapter_id, course_id)
VALUES ('t-python-0000-001', 1, 'print("Hello, 1")', 'python-0000-001', '97931561-7689-44a4-bf80-f2e7c9e8d2dd');

INSERT INTO tutorials (tutorial_id, chapter_number, code_content, chapter_id, course_id)
VALUES ('t-python-0000-002', 2, 'print("Hello, 2")', 'python-0000-002', '97931561-7689-44a4-bf80-f2e7c9e8d2dd');

INSERT INTO tutorials (tutorial_id, chapter_number, code_content, chapter_id, course_id)
VALUES ('t-c-0000-001', 1, '#include <stdio.h>', 'c-0000-001', '10031561-7689-44a4-bf80-f2e7c9e8d2dd');

INSERT INTO tutorials (tutorial_id, chapter_number, code_content, chapter_id, course_id)
VALUES ('t-c-0000-002', 2, '#include <stdio.h>
int main()', 'c-0000-002', '10031561-7689-44a4-bf80-f2e7c9e8d2dd');


INSERT INTO sponsors (uuid, org_name, priority)
VALUES ('08b40464983bcb9522f24f87c76a9e7e', 'ant-group', 1);

INSERT INTO sponsors (uuid, org_name, priority)
VALUES ('26eaa84cc6d2a9f48c8dae60f98fd496', 'dji', 2);

INSERT INTO sponsors (uuid, org_name, priority)
VALUES ('82a4b2c1852f56545d249288ba7e30c4', 'huawei', 3);

INSERT INTO sponsors (uuid, org_name, priority)
VALUES ('92bb00ea903bd360d0fb0294b8728ab4', 'xiaomi', 4);

INSERT INTO sponsors (uuid, org_name, priority)
VALUES ('98aba9d0f5c22c1e3397d2272c25ebd5', 'tencent', 5);

INSERT INTO sponsors (uuid, org_name, priority)
VALUES ('beaaf4670511cb474198e3c1492376cb', 'byd', 6);
