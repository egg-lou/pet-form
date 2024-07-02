import { AxiosService } from '~/composables/AxiosService';

export class PetService extends AxiosService {
    url = '/api/pet';
    async getPets(search: string = '', page_number: number = 1) {
        try {
            const response = await this.request('GET', `${this.url}/get_pets?search=${search}&page=${page_number}`);
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async addPet(pet) {
        try {
            const response = await this.request(
                'POST',
                `${this.url}/add_pet`,
                pet
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async updatePet(pet, pet_id) {
        try {
            const response = await this.request(
                'PATCH',
                `${this.url}/update_pet/${pet_id}`,
                pet
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async deletePet(pet_id) {
        try {
            const response = await this.request(
                'DELETE',
                `${this.url}/delete_pet/${pet_id}`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }
}
