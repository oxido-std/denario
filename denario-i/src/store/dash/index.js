import { createStore } from 'vuex'
import state from '@/store/dash/state'
import getters from '@/store/dash/getters'
import mutations from '@/store/dash/mutations'
import actions from '@/store/dash/actions'

 const DashStore=createStore({
  namespaced:true,
  name:'dashstore',
  state,
  getters,
  mutations,
  actions,
  modules: {
  }
})

export default DashStore