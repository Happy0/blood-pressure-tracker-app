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
    <div>
        <table class="table-auto">
            <tr>
                <th scope="row">Reading</th>
                <th>Weight</th>
                <th>Date</th>
                <th>Time</th>
            </tr>
            <tr v-for="row in readings">
                <td>
                    <table>
                        <tr>
                            <th scope="row">Sys</th>
                            <td>{{ row.systolic }}</td>
                        </tr>
                        <tr>
                            <th scope="row">Dia</th>
                            <td>{{row.diastolic}}</td>
                        </tr>
                        <tr>
                            <th scope="row">Pulse</th>
                            <td>{{ row.pulse }}</td>
                        </tr>
                    </table>
                </td>
                <td>
                    {{ row.weight_kilograms }}
                </td>
                <td>
                    {{ row.date }}
                </td>
                <td>
                    {{ row.time }}
                </td>
            </tr>
        </table>
    </div>
</template>

<style>

</style>