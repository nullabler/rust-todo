CREATE TABLE IF NOT EXISTS task (
    id INT AUTO_INCREMENT NOT NULL,
    category_id INT NOT NULL,
    name VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at DATETIME NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (category_id) REFERENCES category (id)
) DEFAULT CHARACTER SET utf8mb4 COLLATE `utf8mb4_unicode_ci` ENGINE = InnoDB;
