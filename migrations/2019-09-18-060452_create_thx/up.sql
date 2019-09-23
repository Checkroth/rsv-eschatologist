CREATE TABLE thxs(
			 id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
			 channel_id VARCHAR(9) NOT NULL,
			 alias_id INT NOT NULL,
			 CONSTRAINT fk_thx_alias
			 			 FOREIGN KEY (alias_id) REFERENCES aliases (id)
						 ON DELETE CASCADE
						 ON UPDATE RESTRICT
) ENGINE = InnoDB;
