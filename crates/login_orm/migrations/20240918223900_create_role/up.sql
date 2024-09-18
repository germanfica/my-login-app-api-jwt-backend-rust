CREATE TABLE `role` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  -- `name` enum('ROLE_USER','ROLE_ADMIN') NOT NULL,
  -- `name` varchar(60) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `name` varchar(60) NOT NULL,
  PRIMARY KEY (`id`)
);