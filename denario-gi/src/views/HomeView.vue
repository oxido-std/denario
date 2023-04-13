<template>
  <div class="home" v-if="!isBlocked">
    <div class="controls">
        <!-- CONTROLES  -->
        <button @click="monthPrev" class="btn btn-primary">
          ➖
        </button>
        {{ date_month }} / {{ date_year }}
        <button @click="monthNext" class="btn btn-primary">
          ➕
        </button>
        <button @click="loadCurrentDate" class="btn btn-primary">
          Hoy
        </button>
        <button @click="openAddRegisterForm" class="btn btn-success">
          + reg
        </button>
        <select v-model="selectedCategory" @change="loadDateByCategory">
          <option value="0" selected>Todas</option>
          <option v-for="category in categories" :key="category.id" :value="category.id">
            {{ category.name }}
          </option>
        </select>
        <!-- <button @click="loadDateByCategory" class="btn btn-success">
          Ver
        </button> -->
    </div>
  
    <!-- BALANCE -->
    <div class="grid">
      <div class="cell">
        <balance-card title="Ingresos" :amount=total_in color="green"></balance-card>
      </div>
      <div class="cell">
        <balance-card title="Gastos" :amount=total_out color="red"></balance-card>
      </div>
      <div class="cell">
        <balance-card title="Difrencia" :amount=total_difference color="yellow"></balance-card>
      </div>
    </div>
    <hr>
    <!-- MONTH -->
    <div class="grid grid-center">
      <!-- <records-resume></records-resume> -->
    <!-- RESUMEN -->
    <h3>Resumen del mes. Con la opción de ampliar</h3>
    <p>Aquí se ven los subtotales, categorías con gastos e ingresos</p>
    <p>al dejar el mouse sobre el nombre de la tabla se ve el comentario</p>
    <!-- TABLE -->
    <records-table :records=records></records-table>
  </div>
</div>
  <div v-if="isBlocked">
    <record-form></record-form>
  </div>
</template>

<script>

// @ is an alias to /src
import BalanceCard from '@/modules/dash/BalanceCard.vue'
import RecordsTable from '@/modules/dash/RecordsTable.vue'
import RecordForm from '@/modules/dash/RecordForm.vue'
// import RecordsResume from '@/modules/dash/RecordsResume.vue'
// helpers
import {getCurrentDate} from '@/helpers/denario_dates'
// API
import {getBalanceSumAmount,getBalanceAmountWithCategories,getBalanceAmountByCategoryId} from '@/api/balances'
import {getRecordsFromDate,getRecordsFromDateAndCategory} from '@/api/records'
import {getAllCategories} from '@/api/categories'



export default {
  name: 'HomeView',
  components: {
    BalanceCard,
    RecordsTable,
    RecordForm,
    // RecordsResume,
  },
  data(){
    return{
      isBlocked:false,
      total_in:0,
      total_out:0,
      total_difference:0,
      date_month:9,
      date_year:2022,
      records:[],
      categories:[],
      balanceResume:[],
      selectedCategory:0,
    }
  },
  methods:{
    async getCategories(){
      this.categories= await getAllCategories()
    },
    async getDate(){
      const {month,year}=getCurrentDate();
      this.date_month=month
      this.date_year=year
    },
    async getRecords(){
      this.records=await getRecordsFromDate(this.date_month,this.date_year)
    },
    async getRecordsByCategory(){
      if (this.selectedCategory!=0){
        this.records=await getRecordsFromDateAndCategory(this.date_month,this.date_year,this.selectedCategory)
      }else{
        // get all records of the selected month
        this.updateRecords()
      }
    },
    async getBalance(){
      this.total_in=await getBalanceSumAmount("in",this.date_month,this.date_year)
      this.total_out=await getBalanceSumAmount("out",this.date_month,this.date_year)
      this.total_difference=this.total_in-this.total_out;
    },
    async showBalanceAmountIo(balance){
      console.log(balance)
      this.total_in=0;
      this.total_out=0;
      this.total_difference=0;

      for (let index = 0; index < balance.length; index++) {
        const element = balance[index];
        console.log(element.amount_io);
        if (element.amount_io=="in"){
          this.total_in+=element.amount
        }else{
          this.total_out+=element.amount
        }
      }
    },
    async getBalanceWithCategory(){
      const resume_in=await getBalanceAmountWithCategories("in",this.date_month,this.date_year)
      const resume_out= await getBalanceAmountWithCategories("out",this.date_month,this.date_year)
      this.balanceResume=resume_in
      // this.balanceResume['out']=resume_out
      console.log(this.balanceResume)
    },
    monthPrev(){
      if(this.date_month > 1){
        this.date_month=this.date_month-1
      }else{
        this.date_month=12
        this.date_year=this.date_year-1
      }
      this.updateRecords()
    },
    monthNext(){
      if(this.date_month < 12){
        this.date_month=this.date_month+1
      }else{
        this.date_month=1
        this.date_year=this.date_year+1
      }
      this.updateRecords()
    },
    openAddRegisterForm(){
      console.log("add")
      this.isBlocked=true
    },
    closeAddRegisterForm(){
      this.isBlocked=false
    },
    async init(){
      this.getDate()
      await this.getBalance()
      await this.getRecords()
      await this.getCategories()

    },
    async loadCurrentDate(){
      this.getDate()
      this.getBalance()
      await this.getRecords()
    },
    async loadDateByCategory(){
      await this.getRecordsByCategory()
      await this.showBalanceAmountIo(this.records)
    },
    async updateRecords(){
      await this.getBalance()
      await this.getRecords()
    },
  },
  computed:{
  },
  mounted(){
    this.init()
  }

}
</script>
