CREATE TABLE "countries" (
  "id" serial NOT NULL PRIMARY KEY,
  "name" varchar NOT NULL
);

CREATE TABLE "users" (
  "id" serial NOT NULL PRIMARY KEY,
  "email" varchar,
  "first_name" varchar,
  "last_name" varchar,
  "age" integer,
  "country_id" integer,
  "created_at" timestamp without time zone,
  CONSTRAINT "fk-users-countries" FOREIGN KEY ("country_id") REFERENCES "countries" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION
);