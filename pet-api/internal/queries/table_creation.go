package queries

const CreateOwnerTable = `CREATE TABLE IF NOT EXISTS owner (
    owner_id INT AUTO_INCREMENT PRIMARY KEY,
    owner_name VARCHAR(255) NOT NULL,
    owner_address VARCHAR(255) NOT NULL,
    owner_landline_number VARCHAR(255),
    owner_mobile_number VARCHAR(255) NOT NULL,
    owner_email_address VARCHAR(255) NOT NULL
)`

const CreatePetTable = `CREATE TABLE IF NOT EXISTS pet(
    pet_id INT AUTO_INCREMENT PRIMARY KEY,
    owner_id INT NOT NULL,
    pet_name VARCHAR(255) NOT NULL,
    pet_birth_date DATE NOT NULL,
    pet_breed VARCHAR(255) NOT NULL,
    pet_color VARCHAR(255) NOT NULL,
    pet_type VARCHAR(255) NOT NULL,
    pet_weight FLOAT NOT NULL,

    FOREIGN KEY (owner_id) REFERENCES owner(owner_id)
)`

const CreateVeterinarianTable = `CREATE TABLE IF NOT EXISTS veterinarian(
    vet_id INT AUTO_INCREMENT PRIMARY KEY,
    vet_name VARCHAR(255) NOT NULL,
    vet_email_address VARCHAR(50) NOT NULL,
    vet_license_number VARCHAR(255) NOT NULL
)`

const CreateServiceTable = `CREATE TABLE IF NOT EXISTS service(
    service_id INT AUTO_INCREMENT PRIMARY KEY,
    service_name VARCHAR(255) NOT NULL,
    service_cost DECIMAL(10, 2) NOT NULL,
    service_category VARCHAR(255) NOT NULL
)`

const CreateServiceInstanceTable = `CREATE TABLE IF NOT EXISTS service_instance(
    service_instance_id INT AUTO_INCREMENT PRIMARY KEY,
    service_date DATE NOT NULL,
    service_next_date DATE,
    service_follow_up VARCHAR(255) NOT NULL,
    service_reason VARCHAR(255),
    service_notes VARCHAR(255),
    service_diagnosis VARCHAR(255),
    pet_id INT NOT NULL,
    service_id INT NOT NULL,

    FOREIGN KEY (service_id) REFERENCES service(service_id),
    FOREIGN KEY (pet_id) REFERENCES pet(pet_id)
)`

const CreatePreventiveCareTable = `CREATE TABLE IF NOT EXISTS preventive_care(
    preventive_care_id INT AUTO_INCREMENT PRIMARY KEY,
    service_instance_id INT,
	vet_id INT NOT NULL,
    vaccination_type VARCHAR(255),
	
	FOREIGN KEY (vet_id) REFERENCES veterinarian(vet_id),
    FOREIGN KEY (service_instance_id) REFERENCES service_instance(service_instance_id)
)`

const CreateSurgeryTable = `CREATE TABLE IF NOT EXISTS surgery(
    service_instance_id INT PRIMARY KEY,
    anesthesia_used VARCHAR(255),
    complications VARCHAR(255),
    outcome VARCHAR(255),
    vet_id INT NOT NULL,
    
	FOREIGN KEY (vet_id) REFERENCES veterinarian(vet_id),
    FOREIGN KEY (service_instance_id) REFERENCES service_instance(service_instance_id)
)`

const CreateGroomingTable = `CREATE TABLE IF NOT EXISTS grooming(
    grooming_id INT AUTO_INCREMENT PRIMARY KEY,
    service_instance_id INT,
    grooming_type VARCHAR(255),

    FOREIGN KEY (service_instance_id) REFERENCES service_instance(service_instance_id)
)`
