<template>
      <!-- TABLE -->
      <table class="table table-striped table-hover">
      <thead class="thead-dark">
        <tr>
          <!-- <th scope="col">#</th> -->
          <th scope="col">Monto</th>
          <th scope="col">nombre</th>
          <th scope="col">Cat.</th>
          <!-- <th scope="col">Detalle</th> -->
          <th scope="col">Fecha</th>
          <th scope="col"></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="record in records" :key="record.id">
          <!-- <td>{{record.id}}</td> -->
          <td :class=record.amount_io>$ {{record.amount}}</td>
          <td class="tooltip-bottom" data-tooltip="{{ record.comment }}">{{ record.name }}</td>
          <td>{{record.category_name}}</td>
          <!-- <td>{{record.comment}}</td> -->
          <td>{{record.record_date}}</td>
          <td>
            <button @click="console.log('editar')" class="btn btn-warning btn-ghost">
                ✏️
            </button>
            <button @click="console.log('eliminar')" class="btn btn-error btn-ghost">
                ❌
            </button>
          </td>
        </tr>        
      </tbody>
    </table>
</template>

<script>

export default {
    // props:{
    //     records:{
    //         type:Array,
    //     }
    // },
}
</script>

<style scoped>
table{
    font-size: 0.85em;
}
td{
    padding: 2px 0;
    margin:0;
}
.in{
    background-color: #34a853 !important;
    color:#FFF;
}
.out{
    background-color: #ea4335 !important;
    color:#FFF;
}
</style>