import ax from "axios";

export const axios = ax.create({
    baseURL: import.meta.env.VITE_GRAPH_SERVER,
    timeout: 10000,
})
