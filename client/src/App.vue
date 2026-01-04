<script setup lang="ts">
import './assets/main.css'
import 'primeicons/primeicons.css'
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

let loggedIn = ref(false)

const router = useRouter()

type NavItem = {
  label: string
  route: string
  icon: string
}

function isActiveRoute(navItem: NavItem): boolean {
  let currentRoute = router.currentRoute.value.path
  console.log(currentRoute)
  if (currentRoute === '/') {
    return navItem.route === '/'
  } else {
    return navItem.route.startsWith(currentRoute)
  }
}

onMounted(async () => {
  try {
    const res = await fetch('/api/user-info', {
      credentials: 'include',
    })

    if (res.ok) {
      navigationItems.value = loggedInItems
    } else if (res.status === 401) {
      navigationItems.value = loggedOutItems
    }
  } catch (err) {
    console.error('Auth check failed', err)
    loggedIn.value = false
  }
})

const navItems = [
  {
    label: 'Add',
    route: '/',
    icon: '📝',
  },
  {
    label: 'View',
    route: '/view-readings',
    icon: '📖',
  },
]

const loggedOutItems = [
  ...navItems,
  {
    label: 'Login',
    route: '/login',
    icon: '➜]',
  },
]

const loggedInItems = [
  ...navItems,
  {
    label: 'Logout',
    route: '/logout',
    icon: '➜🚪',
  },
]

const navigationItems = ref(loggedInItems)
</script>

<template>
  <div class="flex m-2">
    <div class="max-w-sm mx-auto">
      <div class="bg-white rounded-2xl shadow border">
        <div>
          <div class="flex flex-row">
            <div v-for="item in navigationItems">
              <div
                :class="[
                  'rounded-xl',
                  'shadow',
                  'p-2',
                  'm-2',
                  'justify-center',
                  'hover:bg-sky-300',
                  { 'bg-sky-300': isActiveRoute(item) },
                ]"
              >
                <a :href="item.route">
                  <div class="font-medium">{{ item.label }}</div>
                  <div class="text-center">{{ item.icon }}</div>
                </a>
              </div>
            </div>
          </div>
        </div>
        <main>
          <RouterView></RouterView>
        </main>
      </div>
    </div>
  </div>
</template>

<style></style>
