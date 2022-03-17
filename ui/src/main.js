import { createApp } from 'vue'
// import qs from 'qs'
// import axios from "axios"
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'


//下面是将$axios和$qs挂在原型上，以便在实例中能用 this.$axios能够拿到
//App.prototype.$axios = axios;
// App.prototype.$qs = qs;

const app = createApp(App)

app.use(ElementPlus)
app.mount('#app')
