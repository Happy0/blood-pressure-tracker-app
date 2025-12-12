<script setup lang="ts">

import BloodPressureReadingForm from './components/BloodPressureReadingForm.vue'
import './assets/main.css'
import 'primeicons/primeicons.css'

const now = new Date() 

</script>

<template>
  <h1>You did it!</h1>
  <p>
    Visit <a href="https://vuejs.org/" target="_blank" rel="noopener">vuejs.org</a> to read the
    documentation
  </p>
  <BloodPressureReadingForm :systolic=111 :diastolic=123 :pulse=88 />
  
</template>

<style scoped>

</style>
