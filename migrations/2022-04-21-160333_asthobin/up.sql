CREATE TABLE `asthobin` (
    `id` varchar(10) NOT NULL,
    `content` longtext NOT NULL,
    `time` bigint(20) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `asthobin_id_uindex` (`id`)
);