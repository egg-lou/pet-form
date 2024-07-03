import { AxiosService } from '~/composables/AxiosService';
import type { AddVet, UpdateVet } from '~/types/vet-type';

export class VetService extends AxiosService {
    url = '/api/vet';
    async getVets(page?: number) {
        const limit = 10;
        try {
            const response = await this.request(
                'GET',
                `${this.url}/get_vets?page=${page}&limit=${limit}`
            );

            const { data, status, statusText } = response;
            console.log(response);
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }

    async addVet(vet: AddVet) {
        try {
            const response = await this.request(
                'POST',
                `${this.url}/add_vet`,
                vet
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }

    async updateVet(vet: UpdateVet, vet_id: string) {
        try {
            const response = await this.request(
                'PATCH',
                `${this.url}/update_vet/${vet_id}`,
                vet
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }

    async deleteVet(vet_id: string) {
        try {
            const response = await this.request(
                'DELETE',
                `${this.url}/delete_vet/${vet_id}`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }
}
