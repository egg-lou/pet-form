package queries

import (
	"database/sql"
	"github.com/google/uuid"
	"pet-api/internal/database"
	"time"
)

type OwnerQueries struct {
	BaseQueries
}

type Owner struct {
	ID        uuid.UUID
	Name      string
	Email     string
	Phone     string
	Address   string
	CreatedAt time.Time
	UpdatedAt time.Time
}

func NewOwnerQueries(db database.Service) OwnerQueries {
	baseQueries := NewBaseQueries(db)
	return OwnerQueries{baseQueries}
}
func (oq *OwnerQueries) QueryRow(query string, args ...interface{}) *sql.Row {
	return oq.QueryRow(query, args...)
}

func (oq *OwnerQueries) Create(owner Owner) error {
	owner.ID = uuid.New()
	query := `INSERT INTO owners (owner_id, owner_name, owner_email, owner_phone, owner_address) VALUES (?, ?, ?, ?, ?)`
	_, err := oq.Query(query, owner.ID, owner.Name, owner.Email, owner.Phone, owner.Address)
	return err
}

func (oq *OwnerQueries) Update(owner Owner) error {
	query := `UPDATE owners SET owner_name=?, owner_email=?, owner_phone=?, owner_address=? WHERE owner_id=?`
	_, err := oq.Query(query, owner.Name, owner.Email, owner.Phone, owner.Address, owner.ID)
	return err
}

func (oq *OwnerQueries) Delete(id uuid.UUID) error {
	query := `DELETE FROM owners WHERE owner_id=?`
	_, err := oq.Query(query, id)
	return err
}

func (oq *OwnerQueries) GetAll() ([]Owner, error) {
	query := `SELECT * FROM owners`
	rows, err := oq.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var owners []Owner
	for rows.Next() {
		var owner Owner
		err := rows.Scan(&owner.ID, &owner.Name, &owner.Email, &owner.Phone, &owner.Address, &owner.CreatedAt, &owner.UpdatedAt)
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

func (oq *OwnerQueries) GetById(id uuid.UUID) (*Owner, error) {
	query := `SELECT * FROM owners WHERE owner_id = ?`
	row := oq.QueryRow(query, id)

	var owner Owner
	err := row.Scan(&owner.ID, &owner.Name, &owner.Email, &owner.Phone, &owner.Address, &owner.CreatedAt, &owner.UpdatedAt)
	if err == sql.ErrNoRows {
		return nil, nil
	} else if err != nil {
		return nil, err
	}

	return &owner, nil
}
