<script setup lang="ts">

import BloodPressureReadingForm from './components/BloodPressureReadingForm.vue'
import './assets/main.css'
import 'primeicons/primeicons.css'
import { useRoute } from 'vue-router';

const route = useRoute()

</script>

<template>
  <h1>You did it!</h1>
  <p>
    Visit <a href="https://vuejs.org/" target="_blank" rel="noopener">vuejs.org</a> to read the
    documentation
  </p>
  <div>
      <RouterLink v-if="route.path === '/'" to="/reading" > Take Reading </RouterLink>
    <main>
      <RouterView></RouterView>
    </main>
  </div>
  
</template>

<style scoped>

</style>
