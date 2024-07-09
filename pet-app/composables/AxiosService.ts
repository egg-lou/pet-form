import type { AxiosResponse, Method } from 'axios';
import axios from 'axios';

export class AxiosService {
    private readonly baseURL: string;

    constructor() {
        const config = useRuntimeConfig();
        this.baseURL = config.public.apiUrl;
        axios.defaults.baseURL = this.baseURL;
        axios.defaults.headers.post['Content-Type'] = 'application/json';
        axios.defaults.headers.patch['Content-Type'] = 'application/json';
    }

    async request(
        method: Method,
        url: string,
        data: Record<string, unknown> = {}
    ): Promise<AxiosResponse> {
        try {
            return await axios({
                method,
                url,
                data
            });
        } catch (error) {
            const axiosError = error;
            console.error(axiosError);
            throw axiosError;
        }
    }
}
