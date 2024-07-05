export interface Owner {
    owner_id: number;
    owner_name: string;
    owner_phone_number: string;
    owner_email: string;
    owner_address: string;
}

export interface AddOwner {
    owner_name: string;
    owner_phone_number: string;
    owner_email: string;
    owner_address: string;
}
export interface UpdateOwner {
    owner_name?: string;
    owner_phone_number?: string;
    owner_email?: string;
    owner_address?: string;
}
