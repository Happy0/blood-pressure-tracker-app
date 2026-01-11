import ReadingFromCamera from '@/components/ReadingFromCamera.vue'
import BloodPressureReadingForm from '../components/BloodPressureReadingForm.vue'
import { createRouter, createWebHistory } from 'vue-router'
import ReadingsView from '@/components/view-readings/ReadingsView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: BloodPressureReadingForm },
    {
      path: '/reading-with-values/systolic/:systolic/diastolic/:diastolic/pulse/:pulse',
      component: BloodPressureReadingForm,
      props: true,
    },
    { path: '/reading-from-camera', component: ReadingFromCamera },
    { path: '/view-readings', component: ReadingsView },
  ],
})

export default router
