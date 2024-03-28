CREATE TABLE IF NOT EXISTS user_profiles(
  user_id INT NOT NULL,
  name VARCHAR(255),
  surname VARCHAR(255),
  photo VARCHAR(255),
  phone_number VARCHAR(255),
  FOREIGN KEY (user_id) REFERENCES users(id)
)
