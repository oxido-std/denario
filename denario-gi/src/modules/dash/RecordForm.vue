<template>
    <h1>Agregar registro</h1>
    <form class="form" :class="class_form_error">
        <label class="form-group form-info">
            <input type="text" name="name" required placeholder="Nombre" class="form-control" v-model="record_name">
            <span class="form-label">Nombre</span>
        </label>
        <label class="form-group form-info">
            <select name="amount_io" class="form-control"  v-model="amount_io">
                <option value="out">Egreso</option>
                <option value="in">Ingreso</option>
            </select>
            <span class="form-label">Tipo</span>
        </label>
        <label class="form-group form-info">
            <input type="text" name="amount" required placeholder="$" class="form-control"  v-model="amount">
            <span class="form-label">Monto</span>
        </label>
        <label class="form-group form-info">
            <textarea rows="2" name="comment" required  placeholder="Comentario" class="form-control" v-model="comment"></textarea>
            <span class="form-label">Comentario</span>
        </label>
        <label class="form-group form-info">
            <input type="text" name="record_date" required placeholder="fecha" class="form-control" v-model="record_date">
            <span class="form-label">Fecha</span>
        </label>
        <label class="form-group form-info">
            <select name="category_id" class="form-control" v-model="category_id">
                <option v-for="category in categories_list" :key="category.id" :value="category.id">{{category.name}}</option>
            </select>
            <span class="form-label">Categoría</span>
        </label>
    </form>
    <button @click="saveIntoBackend" class="btn btn-success">
      Guardar
    </button>
    <button class="btn btn-error" @click="this.$emit(closeAddRegisterForm)">
      Cancelar
    </button>
</template>

<script>
import { ref } from 'vue'
import { getCurrentDate,getCurrentDateFull } from '@/helpers/denario_dates';
import { getAllCategories } from '@/api/categories';
import { addRecord } from '@/api/records';

export default {
    data(){
        return {
            record_name:"Prueba",
            amount_io:"out",
            amount:10,
            comment:"-",
            record_date:"2023-03-01",
            category_id:"23",
            is_mutable:true,
            categories_list:[],
            class_form_error:"",
        }

    },
    methods:{
        resetForm(){
            this.record_name=""
            this.amount_io="out"
            this.amount=0.0
            this.comment="-"
            this.record_date=getCurrentDateFull();
            this.category_id="23",
            this.class_form_error=""
        },
        // create a method to save the values of the form to send to the backend
        saveIntoBackend(){
            // this.showValues();
            if(this.formIsValid()){
                console.log("Form is valid")
                addRecord(this.record_name, this.amount, this.amount_io, this.comment, this.record_date, this.category_id,this.is_mutable);
                this.resetForm();
            }else{
                console.log("Form is not valid")
                this.class_form_error="form-error"
            }

        },
        // print in console every value of the form
        showValues(){
            console.log(this.record_name)
            console.log(this.amount_io)
            console.log(this.amount)
            console.log(this.comment)
            console.log(this.record_date)
            console.log(this.category_id)
        },
        // generate a function to check if the form is valid
        formIsValid(){
            if(this.record_name=="" || this.amount==0.0 || this.comment=="" || this.record_date==""){
                return false;
            }
            return true;
        },

    },
    async mounted(){
        this.categories_list=await getAllCategories();
        this.record_date=getCurrentDateFull();
    }
}
</script>

<style>

</style>