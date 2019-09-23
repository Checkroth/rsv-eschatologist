CREATE TABLE user_aliases(
			 id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
			 name_alias VARCHAR(255) NOT NULL,
			 slack_user_id INT,
			 CONSTRAINT fk_alias_user
			 			 FOREIGN KEY (slack_user_id) REFERENCES slack_users (id)
						 ON DELETE CASCADE
						 ON UPDATE RESTRICT
) ENGINE = InnoDB;
