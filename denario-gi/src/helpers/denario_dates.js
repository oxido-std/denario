export const getCurrentDate=()=>{
    const d = new Date();
    return {
        month:(d.getMonth())+1,
        year:d.getFullYear()
    }
}

export const getCurrentDateFull= ()=>{
    const d=new Date();
    
    let month=(d.getMonth())+1;
    const year=d.getFullYear();
    const day=d.getDate();
    const hour=d.getHours();
    const minutes=d.getMinutes();
    const seconds=d.getSeconds();

    let month_fixed=month;
    
    if(month_fixed < 10){
        month_fixed=`0${month_fixed}`
    }
    
    return `${year}-${month_fixed}-${day} ${hour}:${minutes}:${seconds}`
}