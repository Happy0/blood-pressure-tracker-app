import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import PrimeVue from 'primevue/config';
import Lara from '@primeuix/themes/lara';
import Button from "primevue/button"
import { Form } from '@primevue/forms';
import {InputText} from 'primevue'
import DatePicker from 'primevue/datepicker';

import Column from 'primevue/column';
import Select from 'primevue/select';
import Tabs from 'primevue/tabs';

const app = createApp(App)

app.use(router)

app.use(PrimeVue, {
    theme: {
        preset: Lara,
        options: {
            prefix: 'p',
            darkModeSelector: 'system',
            cssLayer: false
        }
    }
})

app.component('Button', Button)
app.component('InputText', InputText)
app.component('Form', Form)
app.component('DatePicker', DatePicker)
app.component('Column', Column)
app.component('Select', Select)
app.component('Tabs', Tabs)

app.mount('#app')
