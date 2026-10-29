-- Your SQL goes here
CREATE TABLE addresses (
  id            INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  user_id       INT NOT NULL,
  receiver_name VARCHAR(100) NOT NULL,
  phone         VARCHAR(10)  NOT NULL,
  province      VARCHAR(100) NOT NULL,
  district      VARCHAR(100) NULL,
  ward          VARCHAR(100) NOT NULL,
  street        VARCHAR(255) NOT NULL,
  is_default    BOOLEAN NOT NULL DEFAULT FALSE,
  created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_addresses_user FOREIGN KEY (user_id)
    REFERENCES users(id) ON DELETE CASCADE,
  INDEX idx_addresses_user_id (user_id)
);