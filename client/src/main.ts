import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import Button from "primevue/button"
import { Form } from '@primevue/forms';
import {InputText} from 'primevue'
import DatePicker from 'primevue/datepicker';

import Column from 'primevue/column';
import Select from 'primevue/select';

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
app.component('Column', Column)
app.component('Select', Select)

app.mount('#app')
