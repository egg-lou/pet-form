export interface Vet {
    vet_id: string;
    vet_name: string;
    vet_email: string;
    vet_phone_number: string;
    vet_license_number: string;
}

export interface AddVet {
    vet_name: string;
    vet_email: string;
    vet_phone_number: string;
    vet_license_number: string;
}

export interface UpdateVet {
    vet_name?: string;
    vet_email?: string;
    vet_phone_number?: string;
    vet_license_number?: string;
}
