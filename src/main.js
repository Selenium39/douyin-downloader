import Vue from 'vue'
import App from './App.vue'
import { Button,Input,Select,Option,Col,Row,Table,TableColumn,Image} from 'element-ui';

Vue.use(Button)
Vue.use(Input)
Vue.use(Select)
Vue.use(Option)
Vue.use(Col)
Vue.use(Row)
Vue.use(Table)
Vue.use(TableColumn)
Vue.use(Image)

Vue.config.productionTip = false

new Vue({
  render: h => h(App),
}).$mount('#app')
