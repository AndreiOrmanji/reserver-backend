CREATE TABLE `countries` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) NOT NULL
);

CREATE TABLE `users` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `email` VARCHAR(255),
  `first_name` VARCHAR(255),
  `last_name` VARCHAR(255),
  `country_id` INT,
  `created_at` DATETIME
);

ALTER TABLE
  `users`
ADD
  CONSTRAINT `fk-users-countries` FOREIGN KEY (`country_id`) REFERENCES `countries`(`id`);

CREATE TABLE `delivery_centers` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR NULL,
  `country_id` INT NULL,
  `created_at` datetime NULL
);

ALTER TABLE
  `delivery_centers`
ADD
  CONSTRAINT `fk-delivery-centers-countries` FOREIGN KEY (`country_id`) REFERENCES `countries`(`id`);

CREATE TABLE `delivery_center_floors` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR NULL,
  `delivery_center_id` INT NULL,
  `created_at` datetime NULL
);

ALTER TABLE
  `delivery_center_floors`
ADD
  CONSTRAINT `fk-delivery-center-floors-countries` FOREIGN KEY (`delivery_center_id`) REFERENCES `delivery_centers`(`id`);

CREATE TABLE `work_desks` (
  `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `floor_id` INT NULL,
  `name` VARCHAR NULL,
  `location_x` INT NULL,
  `location_y` INT NULL,
  `is_available` TINYINT(1) NOT NULL DEFAULT 0,
  `created_at` DATETIME NULL
);

ALTER TABLE
  `work_desks`
ADD
  CONSTRAINT `fk-work-desks-delivery-center-floors` FOREIGN KEY (`floor_id`) REFERENCES `delivery_center_floors`(`id`);