import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import Button from "primevue/button"
import { Form } from '@primevue/forms';
import {InputText} from 'primevue'
import DatePicker from 'primevue/datepicker';

import DataTable from 'primevue/datatable';
import Column from 'primevue/column';

const app = createApp(App)

app.use(router)

app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
})

app.component('Button', Button)
app.component('InputText', InputText)
app.component('Form', Form)
app.component('DatePicker', DatePicker)
app.component('DataTable', DataTable)
app.component('Column', Column)

app.mount('#app')
