import axios from 'axios'
import { apiURL } from './api';

export const getRecordsFromDate=async (month,year)=>{
    const {data} = await axios.get(apiURL+`records/by_date/${month}/${year}`)
    .then( r => r).catch((e)=>{
        console.error(e)
    });

    return data.records;
}

export const addRecord=(name,amount,amount_io,comment,record_date,category_id)=>{
    // newRecord={
    //     "name":"Producto",   
    //     "amount":40.0,
    //     "amount_io":"out",
    //     "comment":"-",
    //     "record_date":"2023-01-26 15:32:00",
    //     "category_id":4
    // }
    const newRecord={
        name,
        amount,
        amount_io,
        comment,
        record_date,
        category_id
    }
    console.log(newRecord)
    // const {data} = axios({
    //     method:'post',
    //     url:apiURL+`records/by_date/`,
    //     headers:{},
    //     data:{
    //         newRecord
    //     }
    // }).then( r => r)
    // .catch((e)=>{
    //     console.error(e)
    // });

    // console.log(data)
    // const {data} = await axios.get(apiURL+`records/by_date/${month}/${year}`)

}