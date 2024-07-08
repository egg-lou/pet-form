export interface ServiceContext {
    followup_date: string | null;
    general_diagnosis: string;
    requires_followup: boolean;
    service_date: string;
    service_instance_id: string;
    service_reason: string;
    service_type: string[];
}

export interface ServiceInstance {
    pet_id: string;
    service_instance_id: string;
    service_date: string;
    followup_date: string | null;
    general_diagnosis: string;
    requires_followup: boolean;
    service_reason: string;
    service_type: string[];
    grooming: Grooming[];
    surgery: Surgery[];
    preventive_care: PreventiveCare[];
}

interface Grooming {
    grooming_id: number;
    grooming_type: string;
}

interface Surgery {
    surgery_id: number;
    surgery_name: string;
    veterinarian_diagnosis: string;
    anesthesia_used: string;
    complications: string;
    outcome: string;
    vet: Vet;
}

interface Vet {
    vet_name: string;
    vet_license_number: string;
    vet_email: string;
    vet_phone_number: string;
}

interface PreventiveCare {
    preventive_care_id: number;
    treatment: string;
    vet: Vet;
}
