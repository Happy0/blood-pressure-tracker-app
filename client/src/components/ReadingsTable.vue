<script lang="ts" setup>
import { onMounted, ref, type Ref } from 'vue';

    import axios from 'axios';

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
            readings.value = result.data;
        })
    })

</script>

<template>
    <DataTable :value="readings" tableStyle="min-width: 50rem">
        <Column field="systolic" header="Systolic"></Column>
        <Column field="diastolic" header="Diastolic"></Column>
        <Column field="pulse" header="Pulse"></Column>
        <Column field="taken" header="Taken"></Column>
    </DataTable>
</template>

<style>

</style>