package queries

import (
	"database/sql"
	"pet-api/internal/models"
)

type Owner struct {
	db *sql.DB
}

func NewOwner(db *sql.DB) *Owner {
	return &Owner{db: db}
}

func (o *Owner) AddOwner(ownerName, ownerAddress, ownerLandlineNumber, ownerMobileNumber, ownerEmailAddress string) (int, error) {
	const query = `INSERT INTO owner (owner_name, owner_address, owner_landline_number, owner_mobile_number, owner_email_address) VALUES (?, ?, ?, ?, ?)`
	_, err := o.db.Exec(query, ownerName, ownerAddress, ownerLandlineNumber, ownerMobileNumber, ownerEmailAddress)
	if err != nil {
		return 0, err
	}

	var id int
	err = o.db.QueryRow("SELECT LAST_INSERT_ID()").Scan(&id)
	return id, err
}

func (o *Owner) GetAllOwners() ([]models.Owner, error) {
	const query = `SELECT * FROM owner`
	rows, err := o.db.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var owners []models.Owner
	for rows.Next() {
		var owner models.Owner
		err = rows.Scan(&owner.OwnerId, &owner.OwnerName, &owner.OwnerAddress, &owner.OwnerLandlineNumber, &owner.OwnerMobileNumber, &owner.OwnerEmailAddress)
		if err != nil {
			return nil, err
		}
		owners = append(owners, owner)
	}

	if err = rows.Err(); err != nil {
		return nil, err
	}

	return owners, nil
}

func (o *Owner) GetOwnerByID(ownerID int) (*sql.Row, error) {
	const query = `SELECT * FROM owner WHERE owner_id = ?`
	return o.db.QueryRow(query, ownerID), nil
}

func (o *Owner) UpdateOwner(ownerID int, ownerName, ownerAddress, ownerLandlineNumber, ownerMobileNumber, ownerEmailAddress string) error {
	const query = `UPDATE owner SET owner_name = ?, owner_address = ?, owner_landline_number = ?, owner_mobile_number = ?, owner_email_address = ? WHERE owner_id = ?`
	_, err := o.db.Exec(query, ownerName, ownerAddress, ownerLandlineNumber, ownerMobileNumber, ownerEmailAddress, ownerID)
	return err
}

func (o *Owner) DeleteOwner(ownerID int) error {
	const query = `DELETE FROM owner WHERE owner_id = ?`
	_, err := o.db.Exec(query, ownerID)
	return err
}
