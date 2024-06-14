package queries

import (
	"database/sql"
	"pet-api/internal/database"
)

type BaseQueries interface {
	GetAll(item interface{}) ([]interface{}, error)
	GetById(id string) (interface{}, error)
	Create(item interface{}) error
	Update(item interface{}) error
	Delete(id string) error
	Query(query string, args ...interface{}) (*sql.Rows, error)
	Close() error
}

type BaseQueriesImpl struct {
	db database.Service
}

func (bq *BaseQueriesImpl) GetAll(item interface{}) ([]interface{}, error) {
	// Implement the method
	return nil, nil
}

func (bq *BaseQueriesImpl) GetById(id string) (interface{}, error) {
	// Implement the method
	return nil, nil
}

func (bq *BaseQueriesImpl) Create(item interface{}) error {
	// Implement the method
	return nil
}

func (bq *BaseQueriesImpl) Update(item interface{}) error {
	// Implement the method
	return nil
}

func (bq *BaseQueriesImpl) Delete(id string) error {
	// Implement the method
	return nil
}

func (bq *BaseQueriesImpl) Query(query string, args ...interface{}) (*sql.Rows, error) {
	return bq.db.Query(query, args...)
}

func (bq *BaseQueriesImpl) Close() error {
	return bq.db.Close()
}

// NewBaseQueries is a factory for creating base queries.
func NewBaseQueries(db database.Service) BaseQueries {
	return &BaseQueriesImpl{db: db}
}
