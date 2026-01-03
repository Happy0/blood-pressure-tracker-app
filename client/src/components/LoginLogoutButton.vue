<template>
  <button
    class="text-center py-3 block m-2 mx-auto rounded-xl border bg-blue-500 text-white font-medium hover:bg-blue-600 transition"
    @click="handleClick"
    :disabled="status === 'loading'"
  >
    <div v-if="status === 'loading'"></div>
    <div v-else-if="status === 'authenticated'">Logout</div>
    <div v-else>Login</div>
  </button>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'

const status = ref('loading') 
// 'loading' | 'authenticated' | 'unauthenticated'

onMounted(async () => {
  try {
    const res = await fetch('/api/user-info', {
      credentials: 'include'
    })

    if (res.ok) {
      status.value = 'authenticated'
    } else if (res.status === 401) {
      status.value = 'unauthenticated'
    } else {
      status.value = 'unauthenticated'
    }
  } catch (err) {
    console.error('Auth check failed', err)
    status.value = 'unauthenticated'
  }
})

function handleClick() {
  if (status.value === 'authenticated') {
    // hit your API logout endpoint
    window.location.href = '/logout'
  } else {
    // start OIDC login
    window.location.href = '/login'
  }
}
</script>

<style scoped>
.login-btn {
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
}
</style>
