import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createI18n } from 'vue-i18n'
import 'tawian-frontend';
import 'typeface-cousine';

// import numberFormat_Denario from '@/helpers/numberformats.js';
// const numberFormat_Denario = require('@/helpers/numberformats.js');

const i18n=createI18n({
    // numberFormats:numberFormat_Denario
})

createApp(App)
    .use(i18n)
    .use(router)
    .mount('#app')
