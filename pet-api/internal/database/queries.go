package database

const createPetTable = `CREATE TABLE IF NOT EXISTS pets (
    pet_id INT AUTO_INCREMENT,
    pet_name VARCHAR(100) NOT NULL,
    pet_type VARCHAR(100) NOT NULL,
    pet_breed VARCHAR(100) NOT NULL,
    pet_age INT NOT NULL,
    pet_weight DECIMAL(5, 2) NOT NULL,
    pet_height DECIMAL(5, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    owner_id INT NOT NULL,
    PRIMARY KEY (pet_id),
    FOREIGN KEY (owner_id) REFERENCES owners(owner_id)    
);`

const createOwnerTable = `CREATE TABLE IF NOT EXISTS owners(
    owner_id INT AUTO_INCREMENT,
    owner_name VARCHAR(100) NOT NULL,
    owner_email VARCHAR(100) NOT NULL,
    owner_phone VARCHAR(100) NOT NULL,
    owner_address VARCHAR(300) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (owner_id)
);`

const createVetTable = `CREATE TABLE IF NOT EXISTS vets(
    veterinarian_id INT AUTO_INCREMENT,
    veterinarian_name VARCHAR(100) NOT NULL,
    veterinarian_email VARCHAR(100) NOT NULL,
    veterinarian_phone VARCHAR(100) NOT NULL,
    PRIMARY KEY (veterinarian_id)
);`
