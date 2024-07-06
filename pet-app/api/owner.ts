import { AxiosService } from '~/composables/AxiosService';
import type { AddOwner, UpdateOwner } from '~/types/owner-type';
import type { RouteParamValue } from 'vue-router';

export class OwnerService extends AxiosService {
    url = '/api/owner';

    async getOwners(search: string = '', page_number: number = 1) {
        const limit = 5;
        try {
            const response = await this.request(
                'GET',
                `${this.url}/get_owners?search=${search}&page=${page_number}&limit=${limit}`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }

    async addOwner(owner: AddOwner) {
        try {
            const response = await this.request(
                'POST',
                `${this.url}/add_owner`,
                owner
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }
    async updateOwner(owner: UpdateOwner, owner_id: String | undefined) {
        try {
            const response = await this.request(
                'PATCH',
                `${this.url}/update_owner/${owner_id}`,
                owner
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }
    async deleteOwner(owner_id: string | RouteParamValue[]) {
        try {
            const response = await this.request(
                'DELETE',
                `${this.url}/delete_owner/${owner_id}`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }

    async getOwnerAndPets(
        owner_id: InferPropType<StringConstructor> | undefined
    ) {
        try {
            const response = await this.request(
                'GET',
                `${this.url}/get_owner_and_pets/${owner_id}`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }
}
