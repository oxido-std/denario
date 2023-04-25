import axios from 'axios'
import { apiURL } from './api';

export const getRecordsFromDate=async (month,year)=>{
    const {data} = await axios.get(apiURL+`records/by_date/${month}/${year}`)
                    .then( r => r).catch((e)=>{
                        console.error(e)
                    });

    return data.records;
}

export const getRecordsFromDateAndCategory=async (month,year,category_id)=>{

    if (month<10) month='0'+month;

    const {data} = await axios.get(apiURL+`records/by_category/${category_id}/date/${month}/${year}`)
                    .then( r => r).catch((e)=>{
                        console.error(e)
                    });

    return data.records;
}

export const addRecord=async (name,amount,amount_io,comment,record_date,category_id,is_mutable)=>{
    const newRecord={
        name,
        amount:parseFloat(amount),
        amount_io,
        comment,
        record_date,
        category_id:parseInt(category_id),
        is_mutable,
    }

    console.log(newRecord)

    //send data to api with axios post method on the body json format

    const {data}= await axios({
        method: 'post',
        url: apiURL+`records`,
        headers:{},
        data: newRecord,
    }).then( r => r).catch((e)=>{
        console.error(e)
    });

    return data;
}