<script lang="ts" setup>
import { onMounted, ref, type Ref } from 'vue';

    import axios from 'axios';
    import {DateTime} from 'luxon';

    type Reading = {
        systolic: number,
        diastolic: number,
        pulse: number,
        taken: string
    }

    const readings: Ref<Reading[]> = ref([])

    onMounted(() => {
        const now = new Date(Date.now());
        const dateFrom = new Date(now);
        dateFrom.setFullYear(now.getFullYear() - 1);

        // TODO: error handling
        axios.get<Reading[]>(`/api/reading?from_inclusive=${dateFrom.toISOString()}&to_inclusive=${now.toISOString()}`).then(result => {
            const rows = result.data.map(row => {
                const date = DateTime.fromISO(row.taken);

                return {
                    ...row,
                    date: date.toISODate(),
                    time: date.toLocaleString(DateTime.TIME_24_SIMPLE)
                } 
            })

            readings.value = rows;
        })
    })

</script>

<template>
    <DataTable :value="readings">
        <Column field="systolic" header="Sys"></Column>
        <Column field="diastolic" header="Dia"></Column>
        <Column field="pulse" header="Pulse"></Column>
        <Column field="date" header="Date"></Column>
        <Column field="time" header="Time"></Column>
    </DataTable>
</template>

<style>

</style>