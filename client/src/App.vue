<script setup lang="ts">

import './assets/main.css'
import 'primeicons/primeicons.css'
import { useRoute } from 'vue-router';
import LoginLogoutButton from './components/LoginLogoutButton.vue';

const route = useRoute()
 

</script>

<template>
  <div v-if="route.path === '/'" class="flex flex-col items-center justify-center min-h-screen bg-gray-50 p-6">
    <div class="bg-white shadow-lg rounded-2xl p-8 w-full max-w-sm flex flex-col gap-4">

      <RouterLink
        to="/reading"
        class="text-center py-3 rounded-xl border bg-blue-500 text-white font-medium hover:bg-blue-600 transition"
      >
        Take Reading
      </RouterLink>

      <RouterLink
        to="/view-readings"
        class="text-center py-3 rounded-xl border bg-blue-500 text-white font-medium hover:bg-blue-600 transition"
      >
        View Readings
      </RouterLink>

      <LoginLogoutButton/>
    </div>
  </div>
  <main>
    <RouterView></RouterView>
  </main>
</template>

<style scoped>

</style>
