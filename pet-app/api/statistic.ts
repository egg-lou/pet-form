import { AxiosService } from '~/composables/AxiosService';

export class StatisticService extends AxiosService {
    url = '/api/statistics';

    async getServicesCounter() {
        try {
            const response = await this.request(
                'GET',
                `${this.url}/counter_services`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async getServicesPetTypeSummary() {
        try {
            const response = await this.request(
                'GET',
                `${this.url}/get_pet_type_visit_summary`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }
}
