<template>
  <header class="site-header dashed-bottom">
    <a class="site-title site-title-2">
      <img src="../assets/logo.png" height="32">
      Denario</a>
    <nav class="site-nav nav-separated">
      <form class="site-search" action="." method="GET">
          <input type="search" name="search" placeholder="search">
      </form>
      <ul>
          <li><a href="#"><router-link to="/">Home</router-link></a></li>
          <li><a href="#"><router-link to="/about">About</router-link></a></li>
      </ul>
    </nav>
  </header>
</template>
<!-- <router-link to="/">Home</router-link> -->

<script>
export default {

}
</script>

<style scoped>
header{
  margin: 0 0 10px 0 !important;
  padding: 5px 10px !important;
  height: 80px;
}
.site-title-2{
  font-size: 1.8em;
  margin: 0 0 10px 0;
}

</style>