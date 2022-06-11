-- Your SQL goes here

CREATE TABLE todos(
   id INT GENERATED ALWAYS AS IDENTITY,
   user_id INT,
   task VARCHAR(255) NOT NULL,
   is_completed BOOL default false,
   PRIMARY KEY(id),
   CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES users(id)
);
