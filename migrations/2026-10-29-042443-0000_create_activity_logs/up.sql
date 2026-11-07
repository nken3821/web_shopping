-- Your SQL goes here
CREATE TABLE activity_logs (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    actor_id INT NULL,
    actor_role VARCHAR(20) NULL,
    action VARCHAR(100) NOT NULL,
    target_type VARCHAR(50) NULL,
    target_id VARCHAR(100) NULL,
    description VARCHAR(255) NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'SUCCESS',
    ip_address VARCHAR(45) NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    INDEX idx_actor_id (actor_id),
    INDEX idx_action (action),
    INDEX idx_created_at (created_at),
    INDEX idx_target (target_type, target_id)
);
