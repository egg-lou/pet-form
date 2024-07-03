import { AxiosService } from '~/composables/AxiosService';

export class VetService extends AxiosService {
    url = '/api/vet';
    async getVets() {
        try {
            const response = await this.request('GET', `${this.url}/get_vets`);

            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }
}
