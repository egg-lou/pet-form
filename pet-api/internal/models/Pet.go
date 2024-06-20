package models

type Pet struct {
	PetId        int    `json:"pet_id"`
	PetName      string `json:"pet_name"`
	PetType      string `json:"pet_type"`
	PetBreed     string `json:"pet_breed"`
	PetColor     string `json:"pet_color"`
	PetBirthDate string `json:"pet_birth_date"`
	OwnerId      int    `json:"owner_id"`
}
