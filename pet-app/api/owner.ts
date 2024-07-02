import { AxiosService } from '~/composables/AxiosService';

export class OwnerService extends AxiosService {
    async getOwners() {
        try {
            const response = await this.request('GET', '/api/owner/get_owners');
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }
}
