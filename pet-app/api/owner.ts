import { AxiosService} from "~/composables/AxiosService";

export class OwnerService extends AxiosService{

    async getOwners() {
        try {
            return await this.request('GET', '/api/owner/get_owners')
        } catch (error) {
            console.error(error);
            throw await Promise.reject(error);
        }
    }
}