import axios from 'axios';

const request = axios.create({});
request.interceptors.response.use((res) => res.data);
export default request;
