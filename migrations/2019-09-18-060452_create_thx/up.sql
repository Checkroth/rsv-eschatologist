CREATE TABLE thxs(
			 id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
			 slack_user_id INT NOT NULL,
			 channel_id VARCHAR(9) NOT NULL,
			 CONSTRAINT fk_thx_user
			 			 FOREIGN KEY (slack_user_id) REFERENCES slack_users (id)
						 ON DELETE CASCADE
						 ON UPDATE RESTRICT
)
