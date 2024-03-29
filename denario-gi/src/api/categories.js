import axios from 'axios'
import { apiURL } from './api'

export const getAllCategories=async ()=>{
    const {data} = await axios.get(apiURL+`categories`)
    .then( r => r).catch((e)=>{
        console.error(e)
    });
    return data.categories;
}

