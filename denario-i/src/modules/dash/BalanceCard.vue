<template>
  <div :class="color">
    <h4>${{ amount }}</h4>
    <i>{{ title }}</i>
  </div>
</template>

<script>
export default {
    props:{
        title:{
            type:String,
            required:true,
            default:"Title"
        },
        amount:{
            type:Number,
            default:0
        },
        color:{
            type:String,
            default:"light"
        }
    }

}
</script>

<style scoped>
div {
    padding: 10px 5px 0 5px;
    margin: 0 0px;
}
div h4{
    font-size:1.20em;
    margin: 0px;
    padding: 0px;
}
div i{
    font-size:0.9em;
    margin: 0px;
    padding: 0px;
}
.green{
    background-color: #34a853;
    color:#FFF;
}
.yellow{
    background-color: #fbbc04;
    color:#FFF;
}
.red{
    background-color: #ea4335;
    color:#FFF;
}
.blue{
    background-color: #46bdc6;
    color:#FFF;
}
.light{
    background-color: #efefef;
    color:#2e2e2e;
}

</style>