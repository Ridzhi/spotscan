import { DefaultApi, Configuration } from "@/utils/openapi";
import {retrieveRawInitData} from "@telegram-apps/sdk-vue";
import globalAxios from "axios";

globalAxios.interceptors.request.use(async (config) => {
    config.headers.Authorization = `tma ${retrieveRawInitData()}`;

    return config;
});

const configuration = new Configuration({
    basePath: import.meta.env.VITE_API_URL,
});

const api = new DefaultApi(configuration);

export {api};