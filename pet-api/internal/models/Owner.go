package models

type Owner struct {
	OwnerId             int    `json:"owner_id"`
	OwnerName           string `json:"owner_name"`
	OwnerAddress        string `json:"owner_address"`
	OwnerLandlineNumber string `json:"owner_landline_number"`
	OwnerMobileNumber   string `json:"owner_mobile_number"`
	OwnerEmailAddress   string `json:"owner_email_address"`
}
