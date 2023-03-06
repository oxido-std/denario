export const actionExample= async({commit})=>{
    commit('setLoading',true)
    // const r =await getRandomInt()
    // commit('icrementVal',r)
    commit('setLoading',false)
    console.log('Example action')
}