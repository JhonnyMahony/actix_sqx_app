CREATE TABLE IF NOT EXISTS user_profiles(
  user_id INT NOT NULL,
  name VARCHAR(255) NULL,
  surname VARCHAR(255) NULL,
  photo VARCHAR(255) NULL,
  phone_number VARCHAR(255) NULL,
  FOREIGN KEY (user_id) REFERENCES users(id)
)
