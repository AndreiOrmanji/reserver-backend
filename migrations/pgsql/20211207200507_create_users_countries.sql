  CREATE TABLE "countries" (
    "id" serial NOT NULL PRIMARY KEY,
    "name" varchar NOT NULL
  );

  CREATE TABLE "users" (
    "id" serial4 NOT NULL,
    "email" varchar NULL,
    "first_name" varchar NULL,
    "last_name" varchar NULL,
    "age" int4 NULL,
    "country_id" int4 NULL,
    "created_at" timestamp NULL,
    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
  );

  ALTER TABLE
    "users"
  ADD
    CONSTRAINT "fk-users-countries" FOREIGN KEY ("country_id") REFERENCES "countries"("id");

  CREATE TABLE "delivery_centers" (
    "id" serial4 NOT NULL,
    "name" varchar NULL,
    "country_id" int4 NULL,
    "created_at" timestamp NULL,
    CONSTRAINT "delivery_centers_pkey" PRIMARY KEY ("id")
  );

  ALTER TABLE
    "delivery_centers"
  ADD
    CONSTRAINT "fk-delivery-centers-countries" FOREIGN KEY ("country_id") REFERENCES "countries"("id");

  CREATE TABLE "delivery_center_floors" (
    "id" serial4 NOT NULL,
    "name" varchar NULL,
    "delivery_center_id" int4 NULL,
    "created_at" timestamp NULL,
    CONSTRAINT "delivery_center_floors_pkey" PRIMARY KEY ("id")
  );

  ALTER TABLE
    "delivery_center_floors"
  ADD
    CONSTRAINT "fk-delivery-center-floors-countries" FOREIGN KEY ("delivery_center_id") REFERENCES "delivery_centers"("id");

  CREATE TABLE "work_desks" (
    "id" serial4 NOT NULL,
    "floor_id" int4 NULL,
    "name" varchar NULL,
    "location_x" int4 NULL,
    "location_y" int4 NULL,
    "is_available" bool NOT NULL DEFAULT false,
    "created_at" timestamp NULL,
    CONSTRAINT "work_desks_pkey" PRIMARY KEY ("id")
  );

  ALTER TABLE
    "work_desks"
  ADD
    CONSTRAINT "fk-work-desks-delivery-center-floors" FOREIGN KEY ("floor_id") REFERENCES "delivery_center_floors"("id");