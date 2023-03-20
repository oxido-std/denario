<template>
    <div class="card">
    <div>
        <header class="card-header" v-bind:class="color">${{ amount }}</header>
        <i>{{ title }}</i>
    </div>
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
.green{
    background-color: #34a853 !important;
    color:#FFF;
}
.yellow{
    background-color: #fbbc04 !important;
    color:#FFF;
}
.red{
    background-color: #ea4335 !important;
    color:#FFF;
}
.blue{
    background-color: #46bdc6 !important;
    color:#FFF;
}
.light{
    background-color: #efefef !important;
    color:#2e2e2e;
}

</style>