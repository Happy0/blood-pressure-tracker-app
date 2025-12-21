<script setup lang="ts">

import './assets/main.css'
import 'primeicons/primeicons.css'
import { useRoute } from 'vue-router';
import axios from 'axios'
import LoginLogoutButton from './components/LoginLogoutButton.vue';

const route = useRoute()

const axiosInstance = axios.create();
axiosInstance.defaults.maxRedirects = 0; 

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

      <button
        disabled
        class="py-3 rounded-xl border border-gray-300 text-gray-400 bg-gray-100 cursor-not-allowed"
      >
        View Readings (coming soon)
      </button>

      <LoginLogoutButton/>
    </div>
  </div>
  <main>
    <RouterView></RouterView>
  </main>
</template>

<style scoped>

</style>
