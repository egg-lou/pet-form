-- Insert query for service_instance
INSERT INTO service_instance (service_instance_id, service_date, service_reason, general_diagnosis, requires_followup, followup_date, pet_id) VALUES (?, ?, ?, ?, ?, ?, ?);

-- Insert query for grooming
INSERT INTO grooming (grooming_type, service_instance_id) VALUES (?, ?);

-- Insert query for preventive_care
INSERT INTO preventive_care (treatment, vet_id, service_instance_id) VALUES (?, ?, ?);

-- Insert query for surgery
INSERT INTO surgery (surgery_name, anesthesia_used, complications, outcome, service_instance_id, vet_id) VALUES (?, ?, ?, ?, ?, ?);

-- Select query for service_instance
SELECT * FROM service_instance WHERE service_instance_id = ?;

-- Select query for grooming
SELECT * FROM grooming WHERE service_instance_id = ?;

-- Select query for preventive_care
SELECT * FROM preventive_care WHERE service_instance_id = ?;

-- Select query for surgery
SELECT * FROM surgery WHERE service_instance_id = ?;

-- Update query for service_instance
SELECT service_type_name FROM service_type WHERE service_instance_id = ?;


-- This query retrieves details of service instances along with the service type name for a specific pet.
-- It performs a LEFT JOIN with the service_type table to include the service type name.
-- The query filters service instances by a specific pet_id.
-- Results are ordered by the service_instance_id and support pagination through LIMIT and OFFSET.
SELECT service_instance.*, service_type.service_type_name
FROM service_instance
         LEFT JOIN service_type ON service_instance.service_instance_id = service_type.service_instance_id
WHERE service_instance.pet_id = ?
ORDER BY service_instance.service_instance_id
    LIMIT ? OFFSET ?
