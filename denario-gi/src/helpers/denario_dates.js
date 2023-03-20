export const getCurrentDate=()=>{
    const d = new Date();
    return {
        month:(d.getMonth())+1,
        year:d.getFullYear()
    }
}