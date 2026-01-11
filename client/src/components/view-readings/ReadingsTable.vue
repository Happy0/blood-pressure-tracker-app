<script lang="ts" setup>
import { onMounted, ref, type Ref } from 'vue'

import axios from 'axios'
import { DateTime } from 'luxon'

type Reading = {
  systolic: number
  diastolic: number
  pulse: number
  taken: string
  weight_kilograms: number | null
}

type ReadingTableRow = {
  systolic: number
  diastolic: number
  pulse: number
  date: string | null
  time: string
  weight_kilograms: number | null
}

const readings: Ref<ReadingTableRow[]> = ref([])

onMounted(() => {
  const now = new Date(Date.now())
  const dateFrom = new Date(now)
  dateFrom.setFullYear(now.getFullYear() - 1)

  // TODO: error handling
  axios
    .get<
      Reading[]
    >(`/api/reading?from_inclusive=${dateFrom.toISOString()}&to_inclusive=${now.toISOString()}`)
    .then((result) => {
      const rows = result.data.map((row) => {
        const date = DateTime.fromISO(row.taken)

        return {
          systolic: row.systolic,
          diastolic: row.diastolic,
          pulse: row.pulse,
          weight_kilograms: row.weight_kilograms,
          date: date.toISODate(),
          time: date.toLocaleString(DateTime.TIME_24_SIMPLE),
        }
      })

      readings.value = rows
    })
})
</script>

<template>
  <div class="overflow-x-auto m-2">
    <table class="min-w-full border border-gray-300 rounded-lg border-collapse text-sm">
      <thead class="bg-gray-100">
        <tr>
          <th scope="col" class="px-4 py-2 text-left font-semibold border border-gray-300">Sys</th>
          <th scope="col" class="px-4 py-2 text-left font-semibold border border-gray-300">Dia</th>
          <th scope="col" class="px-4 py-2 text-left font-semibold border border-gray-300">
            Pulse
          </th>
          <th scope="col" class="px-4 py-2 text-left font-semibold border border-gray-300">
            Weight
          </th>
          <th scope="col" class="px-4 py-2 text-left font-semibold border border-gray-300">
            Taken
          </th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="(row, index) in readings" :key="index" class="odd:bg-white even:bg-gray-50">
          <td class="px-4 py-2 border border-gray-300">
            {{ row.systolic }}
          </td>
          <td class="px-4 py-2 border border-gray-300">
            {{ row.diastolic }}
          </td>
          <td class="px-4 py-2 border border-gray-300">
            {{ row.pulse }}
          </td>
          <td class="px-4 py-2 border border-gray-300">
            {{ row.weight_kilograms }}
          </td>

          <td class="p-2 border border-gray-300 align-top text-xs">
            <!-- Child table -->
            <div>{{ row.date }}</div>
            <div>{{ row.time }}</div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style></style>
