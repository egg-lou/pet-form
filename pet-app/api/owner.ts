import { AxiosService } from '~/composables/AxiosService';

export class OwnerService extends AxiosService {
    async getOwners(search: string = '', page_number: number = 1) {
        try {
            const response = await this.request('GET', `/api/owner/get_owners?search=${search}&page=${page_number}`);
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }
}
