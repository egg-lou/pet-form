package models

type Veterinarian struct {
	VeterinarianId            int    `json:"vet_id"`
	VeterinarianName          string `json:"vet_name"`
	VeterinarianEmailAddress  string `json:"vet_email_address"`
	VeterinarianLicenseNumber string `json:"vet_license_number"`
}
