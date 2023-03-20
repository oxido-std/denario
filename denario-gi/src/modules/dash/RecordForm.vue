<template>
    <h1>Agregar registro</h1>
    <form class="form">
        <label class="form-group form-info">
            <input type="text" name="name" required placeholder="Nombre" class="form-control" :value="name">
            <span class="form-label">Nombre</span>
        </label>
        <label class="form-group form-info">
            <select name="amount_io" class="form-control"  :value="amount_io">
                <option value="out">Egreso</option>
                <option value="in">Ingreso</option>
            </select>
            <span class="form-label">Tipo</span>
        </label>
        <label class="form-group form-info">
            <input type="text" name="amount" required placeholder="$" class="form-control"  :value="amount">
            <span class="form-label">Monto</span>
        </label>
        <label class="form-group form-info">
            <textarea rows="2" name="comment" required  placeholder="Comentario" class="form-control" :value="comment"></textarea>
            <span class="form-label">Comentario</span>
        </label>
        <label class="form-group form-info">
            <input type="text" name="record_date" required placeholder="fecha" class="form-control" :value="record_date">
            <span class="form-label">Fecha</span>
        </label>
        <label class="form-group form-info">
            <select name="category_id" class="form-control" value="category_id">
                <option>Cat 1</option>
                <option>Cat 2</option>
                <option>Cat 3</option>
            </select>
            <span class="form-label">Categoría</span>
        </label>
    </form>
    <button @click="prepareToSave" class="btn btn-success">
      Guardar
    </button>
    <button class="btn btn-error" @click="this.$emit(closeAddRegisterForm)">
      Cancelar
    </button>
</template>

<script>
import { ref } from 'vue'

export default {
    data(){
        return {
            name:"",
            amount_io:"out",
            amount:0.0,
            comment:"-",
            record_date:"2023-01-01",
            category_id:"23",
        }

    },
    methods:{
        resetForm(){
            this.name=""
            this.amount_io="out"
            this.amount=0.0
            this.comment="-"
            this.record_date="2023-01-01"
            this.category_id="23"
        },
        prepareToSave(){
            console.log(this.name)
            console.log(this.amount_io)
            console.log(this.amount)
            console.log(this.comment)
            console.log(this.record_date)
            console.log(this.category_id)
        }
    }
}
</script>

<style>

</style>