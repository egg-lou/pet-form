CREATE TABLE IF NOT EXISTS owner (
    owner_id VARCHAR(36) PRIMARY KEY NOT NULL,
    owner_name VARCHAR(80) NOT NULL,
    owner_email VARCHAR(80),
    owner_phone_number VARCHAR(20),
    owner_address VARCHAR(120),

    UNIQUE (owner_email)
);

CREATE TABLE IF NOT EXISTS pet (
    pet_id VARCHAR(36) PRIMARY KEY NOT NULL,
    pet_name VARCHAR(50) NOT NULL,
    pet_birth_date DATE,
    pet_type ENUM('Dog', 'Cat') NOT NULL,
    pet_breed VARCHAR(40),
    pet_weight DECIMAL(5,2),
    pet_color VARCHAR(20),
    owner_id VARCHAR(36) NOT NULL,

    FOREIGN KEY (owner_id) REFERENCES owner(owner_id)
);

CREATE TABLE IF NOT EXISTS veterinarian (
    vet_id VARCHAR(36) PRIMARY KEY NOT NULL,
    vet_name VARCHAR(80) NOT NULL,
    vet_email VARCHAR(50),
    vet_license_number VARCHAR(20)
);

CREATE TABLE IF NOT EXISTS service_instance (
    service_instance_id VARCHAR(36) PRIMARY KEY NOT NULL,
    service_date DATE NOT NULL,
    service_type ENUM('General Checkup', 'Grooming', 'Surgery', 'Preventive Care'),
    service_reason VARCHAR(255),
    veterinarian_diagnosis VARCHAR(500),
    requires_followup BOOLEAN,
    followup_date DATE,

    pet_id VARCHAR(36) NOT NULL,
    FOREIGN KEY (pet_id) REFERENCES pet(pet_id)
);

CREATE TABLE IF NOT EXISTS grooming (
    grooming_id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    grooming_type VARCHAR(20) NOT NULL,
    service_instance_id VARCHAR(36) NOT NULL,

    FOREIGN KEY (service_instance_id) REFERENCES service_instance(service_instance_id)
);

CREATE TABLE IF NOT EXISTS preventive_care (
    preventive_care_id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    treatment VARCHAR(100) NOT NULL,
    vet_id VARCHAR(36) NOT NULL,
    service_instance_id VARCHAR(36) NOT NULL,

    FOREIGN KEY (vet_id) REFERENCES veterinarian(vet_id),
    FOREIGN KEY (service_instance_id) REFERENCES service_instance(service_instance_id)
);

CREATE TABLE IF NOT EXISTS surgery (
    surgery_id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    surgery_name VARCHAR(50) NOT NULL,
    anesthesia_used VARCHAR(50),
    complications VARCHAR(200),
    outcome VARCHAR(200),
    service_instance_id VARCHAR(36) NOT NULL,
    vet_id VARCHAR(36) NOT NULL,

    FOREIGN KEY (vet_id) REFERENCES veterinarian(vet_id),
    FOREIGN KEY (service_instance_id) REFERENCES service_instance(service_instance_id)
);