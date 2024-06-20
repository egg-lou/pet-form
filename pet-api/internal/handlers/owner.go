package handlers

import (
	"github.com/gofiber/fiber/v2"
	"pet-api/internal/database"
	"pet-api/internal/models"
	"pet-api/internal/queries"
	"strconv"
)

type OwnerHandler struct {
	OwnerQueries *queries.Owner
}

func NewOwnerHandler(db database.Service) *OwnerHandler {
	return &OwnerHandler{
		OwnerQueries: queries.NewOwner(db.GetDb()),
	}
}

func (h *OwnerHandler) SetupRoutes(router fiber.Router) {
	router.Post("/owner", h.AddOwner)
	router.Get("/owner", h.GetAllOwners)
	router.Get("/owner/:id", h.GetOwnerByID)
	router.Put("/owner/:id", h.UpdateOwner)
	router.Delete("/owner/:id", h.DeleteOwner)
}

func (h *OwnerHandler) AddOwner(c *fiber.Ctx) error {
	owner := new(models.Owner)
	if err := c.BodyParser(owner); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
	}

	if owner.OwnerName == "" || owner.OwnerAddress == "" || owner.OwnerMobileNumber == "" || owner.OwnerEmailAddress == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "All fields must be filled"})
	}

	if err := h.OwnerQueries.AddOwner(owner.OwnerName, owner.OwnerAddress, owner.OwnerLandlineNumber, owner.OwnerMobileNumber, owner.OwnerEmailAddress); err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}
	return c.Status(fiber.StatusCreated).JSON(owner)
}

func (h *OwnerHandler) GetAllOwners(c *fiber.Ctx) error {
	owners, err := h.OwnerQueries.GetAllOwners()
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}
	return c.Status(fiber.StatusOK).JSON(owners)
}

func (h *OwnerHandler) GetOwnerByID(c *fiber.Ctx) error {
	ownerId, err := strconv.Atoi(c.Params("id"))
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
	}
	owner, err := h.OwnerQueries.GetOwnerByID(ownerId)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}
	return c.Status(fiber.StatusOK).JSON(owner)
}

func (h *OwnerHandler) UpdateOwner(c *fiber.Ctx) error {
	ownerId, err := strconv.Atoi(c.Params("id"))
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
	}
	owner := new(models.Owner)
	if err = c.BodyParser(owner); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
	}
	if err = h.OwnerQueries.UpdateOwner(ownerId, owner.OwnerName, owner.OwnerAddress, owner.OwnerLandlineNumber, owner.OwnerMobileNumber, owner.OwnerEmailAddress); err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}
	return c.Status(fiber.StatusOK).JSON(owner)
}

func (h *OwnerHandler) DeleteOwner(c *fiber.Ctx) error {
	ownerId, err := strconv.Atoi(c.Params("id"))
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
	}
	if err = h.OwnerQueries.DeleteOwner(ownerId); err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}
	return c.SendStatus(fiber.StatusNoContent)
}
