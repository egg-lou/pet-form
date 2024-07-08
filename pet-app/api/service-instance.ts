import { AxiosService } from '~/composables/AxiosService';
import type { RouteParamValue } from 'vue-router';

export class ServiceInstanceService extends AxiosService {
    url = '/api/service_instance';

    async getPetHistories(
        pet_id: string | RouteParamValue[],
        start_date = '',
        end_date = ''
    ) {
        try {
            const response = await this.request(
                'GET',
                `${this.url}/get_pet_histories/${pet_id}?start_date=${start_date}&end_date=${end_date}`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async deleteServiceInstance(service_instance_id: string) {
        try {
            const response = await this.request(
                'DELETE',
                `${this.url}/delete_service/${service_instance_id}`
            );

            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }
}
