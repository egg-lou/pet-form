package handlers

import (
	"database/sql"
	"github.com/gofiber/fiber/v2"
	"pet-api/internal/queries"
)

type OwnerHandler struct {
    OwnerQueries *queries.Owner
}

func NewOwnerHandler(db *sql.DB) *OwnerHandler {
	return &OwnerHandler {
		OwnerQueries: queries.NewOwner(db),
	}
}

func (h *OwnerHandler SetupRoutes(router fiber.Router) {
	router.Post("/owner", h.AddOwner)
	router.Get("/owner", h.GetAllOwners)
	router.Get("/owner/:id", h.GetOwnerByID)
	router.Put("/owner/:id", h.UpdateOwner)
	router.Delete("/owner/:id", h.DeleteOwner)
})

func (h *OwnerHandler AddOwner(c *fiber.Ctx) error { }
func (h *OwnerHandler GetAllOwners(c *fiber.Ctx) error { }
func (h *OwnerHandler GetOwnerByID(c *fiber.Ctx) error { }
func (h *OwnerHandler UpdateOwner(c *fiber.Ctx) error { }
func (h *OwnerHandler DeleteOwner(c *fiber.Ctx) error { }
