CREATE TABLE IF NOT EXISTS owner (
    ownerId INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    ownerName VARCHAR(80) NOT NULL,
    ownerEmail VARCHAR(80),
    ownerPhoneNumber VARCHAR(20),
    ownerAddress VARCHAR(120)
);

CREATE TABLE IF NOT EXISTS pet (
    petId INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    petNAME VARCHAR(50) NOT NULL,
    petBirthDate DATE,
    petType ENUM('Dog', 'Cat') NOT NULL,
    petBreed VARCHAR(40),
    petWeight DECIMAL(5,2),
    petColor VARCHAR(20),
    ownerId INT NOT NULL,

    FOREIGN KEY (ownerId) REFERENCES owner(ownerId)
);

CREATE TABLE IF NOT EXISTS veterinarian (
    vetId INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    vetName VARCHAR(80) NOT NULL,
    vetEmail VARCHAR(50),
    vetLicenseNumber VARCHAR(20)
);

CREATE TABLE IF NOT EXISTS serviceInstance (
    serviceInstanceId INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    serviceDate DATE NOT NULL,
    serviceType ENUM('General Checkup', 'Grooming', 'Surgery', 'Preventive Care'),
    serviceReason VARCHAR(255),
    veterinarianDiagnosis VARCHAR(500),
    requiresFollowUp BOOLEAN,
    followUpDate DATE,

    petId INT NOT NULL,
    FOREIGN KEY (petId) REFERENCES pet(petId)
);

CREATE TABLE IF NOT EXISTS grooming (
    groomingId INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    groomingType VARCHAR(20) NOT NULL,
    serviceInstanceId INT NOT NULL,

    FOREIGN KEY (serviceInstanceId) REFERENCES serviceInstance(serviceInstanceId)
);

CREATE TABLE IF NOT EXISTS preventiveCare (
    preventiveCareId INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    treatment VARCHAR(100) NOT NULL,
    vetId INT NOT NULL,
    serviceInstanceId INT NOT NULL,

    FOREIGN KEY (vetId) REFERENCES veterinarian(vetId),
    FOREIGN KEY (serviceInstanceId) REFERENCES serviceInstance(serviceInstanceId)
);

CREATE TABLE IF NOT EXISTS surgery (
    surgeryId INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    surgeryName VARCHAR(50) NOT NULL,
    anesthesiaUsed VARCHAR(50),
    complications VARCHAR(200),
    outcome VARCHAR(200),
    serviceInstanceId INT NOT NULL,
    vetId INT NOT NULL,

    FOREIGN KEY (vetId) REFERENCES veterinarian(vetId),
    FOREIGN KEY (serviceInstanceId) REFERENCES serviceInstance(serviceInstanceId)
);

