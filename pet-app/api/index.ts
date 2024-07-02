import {AxiosService} from "~/composables/AxiosService";

export class IndexService extends AxiosService {
    async getIndex() {
        try {
            const response =  await this.request('GET', '/api', {})
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }
    async getHealth() {
        try {
            const response =  await this.request('GET', '/api/health_check', {})
            const { data, status, statusText } = response;
            return { data, status, statusText };
        } catch (error) {
            console.error(error);
            return await Promise.reject(error);
        }
    }
}