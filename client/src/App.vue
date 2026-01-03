<script setup lang="ts">
  import './assets/main.css'
  import 'primeicons/primeicons.css'
  import { useRoute } from 'vue-router';
  import LoginLogoutButton from './components/LoginLogoutButton.vue';

  const route = useRoute()

  const items = [{
    label: "Take Reading",
    route: "/",
    icon:"📝 "
  },
{
  label: "View Readings",
  route: "/view-readings",
  icon: "📖"
}]

</script>

<template>
    <div class="min-h-screen flex items-center justify-center p-4 bg-gray-50">
      <div class="max-w-sm mx-auto">
        <div class="bg-white rounded-2xl shadow p-3 border">

<Tabs value="/">
    <TabList class="flex flex-row mx-auto">
      <span>&nbsp;|&nbsp;</span>
        <Tab v-for="tab in items" :key="tab.label" :value="tab.route">
            <router-link v-if="tab.route" v-slot="{ href, navigate }" :to="tab.route" >
                <a v-ripple :href="href" @click="navigate" class="flex items-center gap-2 text-inherit">
                    <span>{{ tab.icon }}</span>
                    <span>{{ tab.label }}</span>
                    <span> &nbsp;| &nbsp;</span>
                </a>
            </router-link>
        </Tab>
    </TabList>
    </Tabs>
    <hr class="m-2"></hr>
        <main>
          <RouterView></RouterView>
          <div class="max-w-sm mx-auto">
            <LoginLogoutButton></LoginLogoutButton>
          </div>
          
        </main>
        </div>
      </div>
    </div>
</template>

<style>

</style>
