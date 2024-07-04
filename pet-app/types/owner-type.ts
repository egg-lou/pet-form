export interface Owner {
    owner_id: number;
    owner_phone_number: string;
    owner_email: string;
    owner_address: string;
}

export interface AddOwner {
    owner_phone_number: string;
    owner_email: string;
    owner_address: string;
}
export interface UpdateOwner {
    owner_phone_number?: string;
    owner_email?: string;
    owner_address?: string;
}
