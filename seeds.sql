CREATE TABLE users (
    id                integer NOT NULL,
    email             character varying(255) NOT NULL,
    first_name        character varying(255),
    last_name         character varying(255),
    age               integer,
    signed_up_at      timestamp without time zone NOT NULL,
    referred          boolean DEFAULT false NOT NULL,
    bio               text
);


CREATE SEQUENCE users_id_seq
    START     WITH 1
    INCREMENT BY   1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE users_id_seq OWNED BY users.id;

ALTER TABLE ONLY users ALTER COLUMN id SET DEFAULT nextval('users_id_seq'::regclass);


INSERT INTO users (email, first_name, last_name, age, signed_up_at, referred, bio) VALUES
('joe@example.com', 'Joe', 'Johnson', 23, '2015-03-15T12:30:45Z', false, 'Bacon ipsum dolor amet tri-tip picanha turducken meatloaf bacon pig kevin cow pork loin jowl tongue doner landjaeger jerky. Salami shoulder prosciutto venison kevin doner tongue ribeye alcatra. Tail sausage bresaola porchetta pig drumstick jowl bacon flank sirloin spare ribs short loin cow venison. Salami leberkas shoulder frankfurter pig pork chop swine hamburger chicken. Ham meatball tri-tip doner flank biltong capicola frankfurter picanha leberkas beef turkey. Pastrami pancetta leberkas venison, porchetta tongue turducken meatball prosciutto shoulder ham hock sausage beef salami. Pancetta ham hock t-bone meatloaf, tenderloin swine capicola strip steak frankfurter.'),
('john@example.com', 'John', 'Smith', 54, '2016-01-06T14:32:27Z', true, 'Bacon ipsum dolor amet tri-tip picanha turducken meatloaf bacon pig kevin cow pork loin jowl tongue doner landjaeger jerky. Salami shoulder prosciutto venison kevin doner tongue ribeye alcatra. Tail sausage bresaola porchetta pig drumstick jowl bacon flank sirloin spare ribs short loin cow venison. Salami leberkas shoulder frankfurter pig pork chop swine hamburger chicken. Ham meatball tri-tip doner flank biltong capicola frankfurter picanha leberkas beef turkey. Pastrami pancetta leberkas venison, porchetta tongue turducken meatball prosciutto shoulder ham hock sausage beef salami. Pancetta ham hock t-bone meatloaf, tenderloin swine capicola strip steak frankfurter.'),
('minimal@example.com', null, null, 54, '2016-05-05T17:01:23Z', false, null)
;
