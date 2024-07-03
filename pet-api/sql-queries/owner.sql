-- Inserts a new owner record into the owner table
INSERT INTO owner (owner_id, owner_name, owner_email, owner_phone_number, owner_address) VALUES (?, ?, ?, ?, ?)

-- Selects all columns from the owner table where the owner_id matches the specified value
SELECT * FROM owner WHERE owner_id = ?

-- Selects all columns from the owner table where the owner_name matches the specified pattern, ordered by owner_name, with pagination support
SELECT * FROM owner WHERE owner_name LIKE ? ORDER BY owner_name LIMIT ? OFFSET ?

-- Selects all columns from the owner table, ordered by owner_name, with pagination support
SELECT * FROM owner ORDER BY owner_name LIMIT ? OFFSET ?

-- Deletes an owner record from the owner table where the owner_id matches the specified value
DELETE FROM owner WHERE owner_id = ?

-- Selects all columns from the PET table where the owner_id matches the specified value, ordered by pet_name
SELECT * FROM PET WHERE owner_id = ? ORDER BY pet_name