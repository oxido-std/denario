import axios from 'axios'
import { apiURL } from './api';

// example amount_io="in" / "out"
export const getBalanceSumAmount=async (amount_io,month,year)=>{
    const {data} = await axios.get(apiURL+`balances/sum_amount/${amount_io}/${month}/${year}`)
    .then( r => r).catch((e)=>{
        console.error(e)
    });
    return data.records[0].total;
}

export const getBalanceAmountWithCategories=async (amount_io,month,year)=>{
    const {data}= await axios.get(apiURL+`balances/total_amount_with_categories_by/${amount_io}/${month}/${year}`)
    .then( r => r).catch((e)=>{
        console.error(e)
    });
    return data.records;
}
