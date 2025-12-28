<script lang="ts" setup>
import { onMounted, ref, type Ref } from 'vue';

    import axios from 'axios';
    import {DateTime} from 'luxon';

    type Reading = {
        systolic: number,
        diastolic: number,
        pulse: number,
        taken: string,
        weight_kilograms: number | null
    }

    type ReadingTableRow = {
        systolic: number,
        diastolic: number,
        pulse: number,
        date: string | null,
        time: string,
        weight_kilograms: number | null
    }

    const readings: Ref<ReadingTableRow[]> = ref([])

    onMounted(() => {
        const now = new Date(Date.now());
        const dateFrom = new Date(now);
        dateFrom.setFullYear(now.getFullYear() - 1);

        // TODO: error handling
        axios.get<Reading[]>(`/api/reading?from_inclusive=${dateFrom.toISOString()}&to_inclusive=${now.toISOString()}`).then(result => {
            const rows = result.data.map(row => {
                const date = DateTime.fromISO(row.taken);

                return {
                    systolic: row.systolic,
                    diastolic: row.diastolic,
                    pulse: row.pulse,
                    weight_kilograms: row.weight_kilograms,
                    date: date.toISODate(),
                    time: date.toLocaleString(DateTime.TIME_24_SIMPLE)
                } 
            })

            readings.value = rows;
        })
    })

</script>

<template>
  <div class="overflow-x-auto">
    <table class="min-w-full border border-gray-300 rounded-lg border-collapse text-sm">
      <thead class="bg-gray-100">
        <tr>
          <th scope="col" class="px-4 py-2 text-left font-semibold border border-gray-300">
            Reading
          </th>
          <th scope="col" class="px-4 py-2 text-left font-semibold border border-gray-300">
            Weight
          </th>
          <th scope="col" class="px-4 py-2 text-left font-semibold border border-gray-300">
            Date
          </th>
          <th scope="col" class="px-4 py-2 text-left font-semibold border border-gray-300">
            Time
          </th>
        </tr>
      </thead>

      <tbody>
        <tr
          v-for="(row, index) in readings"
          :key="index"
          class="odd:bg-white even:bg-gray-50"
        >
          <td class="p-2 border border-gray-300 align-top">
            <!-- Child table -->
            <table class="w-full border border-gray-200 text-xs">
              <tbody>
                <tr>
                  <th
                    scope="row"
                    class="px-2 py-1 text-left font-medium bg-gray-50 border border-gray-200"
                  >
                    Sys
                  </th>
                  <td class="px-2 py-1 border border-gray-200 text-right">
                    {{ row.systolic }}
                  </td>
                </tr>
                <tr>
                  <th
                    scope="row"
                    class="px-2 py-1 text-left font-medium bg-gray-50 border border-gray-200"
                  >
                    Dia
                  </th>
                  <td class="px-2 py-1 border border-gray-200 text-right">
                    {{ row.diastolic }}
                  </td>
                </tr>
                <tr>
                  <th
                    scope="row"
                    class="px-2 py-1 text-left font-medium bg-gray-50 border border-gray-200"
                  >
                    Pulse
                  </th>
                  <td class="px-2 py-1 border border-gray-200 text-right">
                    {{ row.pulse }}
                  </td>
                </tr>
              </tbody>
            </table>
          </td>

          <td class="px-4 py-2 border border-gray-300">
            {{ row.weight_kilograms }}
          </td>
          <td class="px-4 py-2 border border-gray-300">
            {{ row.date }}
          </td>
          <td class="px-4 py-2 border border-gray-300">
            {{ row.time }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style>

</style>