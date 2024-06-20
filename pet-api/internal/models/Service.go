package models

type Service struct {
	ServiceId       int     `json:"service_id"`
	ServiceName     string  `json:"service_name"`
	ServiceCost     float64 `json:"service_cost"`
	ServiceCategory string  `json:"service_category"`
}

type ServiceInstance struct {
	ServiceInstanceId int    `json:"service_instance_id"`
	ServiceDate       string `json:"service_date"`
	ServiceNextDate   string `json:"service_next_date"`
	ServiceFollowUp   string `json:"service_follow_up"`
	ServiceReason     string `json:"service_reason"`
}
