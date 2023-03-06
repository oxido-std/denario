<template>
  <div class="home">
    <!-- BALANCE -->
    <div class="container">
      <div class="row align-content-center">
        <div class="col-4">
          <div class="row">
            <div class="col-sm-6">
              <balance-card title="Ingresos" :amount="15000.12" color="green"></balance-card>
            </div>
            <div class="col-sm-6">
              <balance-card title="Gastos" :amount="2000.12" color="red"></balance-card>
            </div>
          </div>
          <div class="row">
            <div class="col-sm-12">
              <balance-card title="Diferencia" :amount="2000.0" color="yellow"></balance-card>
            </div>
          </div>
        </div>
        <div class="col-sm-1">
          <balance-card title="Diferencia" :amount="300"></balance-card>
        </div>
        <div class="col-sm-1">
          <balance-card title="Diferencia" :amount="300"></balance-card>
        </div>
        <div class="col-sm-1">
          <balance-card title="Diferencia" :amount="300"></balance-card>
        </div>
      </div>
    </div>
    <!-- TABLE -->
    <table class="table table-striped table-hover">
      <thead class="thead-dark">
        <tr>
          <th scope="col">#</th>
          <th scope="col">First</th>
          <th scope="col">Last</th>
          <th scope="col">Handle</th>
          <th scope="col"></th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th scope="row">1</th>
          <td>Mark</td>
          <td>Otto</td>
          <td>@mdo</td>
          <td>
            <a href="">Edit</a> |
            <a href="">Delete</a>
          </td>
        </tr>
        <tr>
          <th scope="row">2</th>
          <td>Jacob</td>
          <td>Thornton</td>
          <td>@fat</td>
          <td>
            <a href="">Edit</a> |
            <a href="">Delete</a>
          </td>
        </tr>
        <tr>
          <th scope="row">3</th>
          <td colspan="2">Larry the Bird</td>
          <td>@twitter</td>
                    <td>
            <a href="">Edit</a> |
            <a href="">Delete</a>
          </td>
        </tr>
        <tr>
          <th scope="row">2</th>
          <td>Jacob</td>
          <td>Thornton</td>
          <td>@fat</td>
                    <td>
            <a href="">Edit</a> |
            <a href="">Delete</a>
          </td>
        </tr>
        <tr>
          <th scope="row">2</th>
          <td>Jacob</td>
          <td>Thornton</td>
          <td>@fat</td>
                    <td>
            <a href="">Edit</a> |
            <a href="">Delete</a>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
// @ is an alias to /src
import BalanceCard from '@/modules/dash/BalanceCard.vue';

export default {
  name: 'HomeView',
  components: {
    BalanceCard
  }
}
</script>
