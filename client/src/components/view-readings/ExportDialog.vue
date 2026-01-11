<script lang="ts" setup>
    import { ref } from 'vue';

    const millisecondsInMonth = 2629746000;
    const weekAgo = new Date(new Date().getTime() - millisecondsInMonth);

    const dateTo = ref(new Date())
    const dateFrom = ref(weekAgo);

    const visible = ref(false)

    const items = [{
        label: "CSV",
        command: () => {
            downloadCSV()
        }
    }]

    function downloadCSV() {
        // WIP
        alert("Testaroonie")
    }

</script>

<template>
    <div class="flex flex-col container min-w-full items-center m-2">

        <Button label="Export" @click="visible = true" />
        <Dialog v-model:visible="visible" modal header="Export" :style="{ width: '25rem' }">
            <div class="flex items-center gap-4 mb-4">
                <label for="username" class="font-semibold w-24">From</label>
                <DatePicker v-model="dateFrom"/>
            </div>
            <div class="flex items-center gap-4 mb-8">
                <label for="email" class="font-semibold w-24">To</label>
                <DatePicker v-model="dateTo"/>
            </div>
            <div class="flex justify-end gap-2">
                <Button type="button" label="Cancel" severity="secondary" @click="visible = false"></Button>
                <SplitButton type="button" label="Download" @click="downloadCSV" :model="items"></SplitButton>
            </div>
        </Dialog>
        <div>
            
        </div>
    </div>
</template>

<style></style>