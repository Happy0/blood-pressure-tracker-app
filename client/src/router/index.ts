import BloodPressureReadingForm from '../components/BloodPressureReadingForm.vue'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {path: '/reading', component: BloodPressureReadingForm},
    {path: '/reading-with-values/systolic/:systolic/diastolic/:diastolic/pulse/:pulse', component: BloodPressureReadingForm, props: true}
  ],
})

export default router
