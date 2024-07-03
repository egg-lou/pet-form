-- Inserts a new pet record into the pet table with details including ID, name, birth date, type, breed, weight, color, and owner ID
INSERT INTO pet (pet_id, pet_name, pet_birth_date, pet_type, pet_breed, pet_weight, pet_color, owner_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?)

-- Selects all columns from the pet table where the pet_id matches the specified value
SELECT * FROM pet WHERE pet_id = ?

-- Selects all pet details along with the owner's name and email for pets that match a specified name pattern, ordered by pet type, with pagination support
SELECT pet.*, owner.owner_name, owner.owner_email FROM pet INNER JOIN owner ON pet.owner_id = owner.owner_id WHERE pet_name LIKE ? ORDER BY pet_type LIMIT ? OFFSET ?

-- Selects all pet details along with the owner's name and email, ordered by pet type, with pagination support
SELECT pet.*, owner.owner_name, owner.owner_email FROM pet INNER JOIN owner ON pet.owner_id = owner.owner_id ORDER BY pet_type LIMIT ? OFFSET ?

-- Deletes a pet record from the pet table where the pet_id matches the specified value
DELETE FROM pet WHERE pet_id = ?

-- Updates a pet record in the pet table with new details for name, birth date, type, breed, weight, color, and owner ID where the pet_id matches the specified value
UPDATE pet SET pet_name = ?, pet_birth_date = ?, pet_type = ?, pet_breed = ?, pet_weight = ?, pet_color = ?, owner_id = ? WHERE pet_id = ?