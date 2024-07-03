INSERT INTO-- Inserts a new veterinarian record into the veterinarian table
INSERT INTO veterinarian (vet_id, vet_name, vet_email, vet_phone_number, vet_license_number) VALUES (?, ?, ?, ?, ?)

-- Selects all columns from the veterinarian table where the vet_id matches the specified value
SELECT * FROM veterinarian WHERE vet_id = ?

-- Selects all columns from the veterinarian table, ordered by vet_name, with pagination support
SELECT * FROM veterinarian ORDER by vet_name LIMIT ? OFFSET ?

-- Selects only the vet_id and vet_name columns from the veterinarian table
SELECT vet_id, vet_name FROM veterinarian

-- Deletes a veterinarian record from the veterinarian table where the vet_id matches the specified value
DELETE FROM veterinarian WHERE vet_id = ?

-- Updates a veterinarian record in the veterinarian table where the vet_id matches the specified value
UPDATE veterinarian SET vet_name = ?, vet_email = ?, vet_phone_number = ?, vet_license_number = ? WHERE vet_id = ?