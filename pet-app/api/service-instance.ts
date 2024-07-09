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
    async addServiceInstance(service_instance: any) {
        try {
            const response = await this.request(
                'POST',
                `${this.url}/add_service_instance`,
                service_instance
            );

            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async getServiceInstance(service_instance_id: string) {
        try {
            const response = await this.request(
                'GET',
                `${this.url}/get_specific_service_instance/${service_instance_id}`
            );

            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async updateServiceInstance(
        service_instance_id: string,
    service_instance: any,
    ) {
        try {
            const response = await this.request(
                'PATCH',
                `${this.url}/update_service_instance/${service_instance_id}`,
                service_instance
            );

            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async addPreventiveCare(service_instance_id, preventiveCareData) {
        try {
            const response = await this.request(
                'POST',
                `${this.url}/add_preventive_care_to_instance/${service_instance_id}`,
                preventiveCareData
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async addGrooming(service_instance_id, groomingData) {
        try {
            const response = await this.request(
                'POST',
                `${this.url}/add_grooming_to_instance/${service_instance_id}`,
                groomingData
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async deleteGrooming(grooming_id) {
        try {
            const response = await this.request(
                'DELETE',
                `${this.url}/delete_grooming_from_instance/${grooming_id}`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async deletePreventiveCare(preventive_care_id) {
        try {
            const response = await this.request(
                'DELETE',
                `${this.url}/delete_preventive_care_from_instance/${preventive_care_id}`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async deleteSurgery(surgery_id) {
        try {
            const response = await this.request(
                'DELETE',
                `${this.url}/delete_surgery_from_instance/${surgery_id}`
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }

    async updateSurgery(surgery_id, surgeryData) {
        try {
            const response = await this.request(
                'PATCH',
                `${this.url}/update_surgery_from_instance/${surgery_id}`,
                surgeryData
            );
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }
}
