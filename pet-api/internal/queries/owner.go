package queries

import (
	"database/sql"
)

type Owner struct {
	db *sql.DB
}

func NewOwner(db *sql.DB) *Owner {
	return &Owner{db: db}
}

func (o *Owner) AddOwner(ownerName, ownerAddress, ownerLandlineNumber, ownerMobileNumber, ownerEmailAddress string) error {
	const query = `INSERT INTO owner (owner_name, owner_address, owner_landline_number, owner_mobile_number, owner_email_address) VALUES (?, ?, ?, ?, ?)`
	_, err := o.db.Exec(query, ownerName, ownerAddress, ownerLandlineNumber, ownerMobileNumber, ownerEmailAddress)
	return err
}

func (o *Owner) GetAllOwners() (*sql.Rows, error) {
	const query = `SELECT * FROM owner`
	return o.db.Query(query)
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
