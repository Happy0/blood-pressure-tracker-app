<script lang="ts" setup>
import { useRouter } from 'vue-router';

    const router = useRouter();

    const now = new Date()

    const props = defineProps({
        systolic: Number,
        diastolic: Number,
        pulse: Number
        }
    )

    const units = ["Kg"];

    const initial = {
        systolic: props.systolic,
        diastolic: props.diastolic,
        pulse: props.pulse,
        taken: now
    }

    async function onSubmit(params: any) {
        const weightKilograms = params.values.weight === "" ? null: parseFloat(params.values.weight);

        const payload = {
            systolic: parseInt(params.values.systolic),
            diastolic: parseInt(params.values.diastolic),
            pulse: parseInt(params.values.pulse),
            weight_kilograms: weightKilograms,
            taken: params.values.taken.toISOString(),
        };

        // TODO: handle error status codes
        await fetch("/api/reading", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(payload)
        })

        router.push({
            path: `/view-readings`
        })


    }

</script>

<template>
      <div class="min-h-screen flex items-center justify-center p-4 bg-gray-50">
      <div class="max-w-sm mx-auto">
    <div class="bg-white rounded-2xl shadow p-6 border">
    <Form v-slot="$form" :initial-values="initial" @submit="onSubmit" class="flex flex-col gap-4 w-full sm:w-56 mx-auto">
        <div class="flex flex-col gap-1">
            <label class="block mb-1 font-medium">Systolic</Label>
            <InputText name="systolic" type="number" fluid></InputText>
        </div>
        <div class="flex flex-col gap-1">
            <label class="block mb-1 font-medium">Diastolic</Label>
            <InputText name="diastolic" type="number" fluid></InputText>
        </div>
        <div class="flex flex-col gap-1">
            <label class="block mb-1 font-medium">Pulse</Label>
            <InputText name="pulse" type="number" fluid></InputText>
        </div>
        <div class="flex flex-col gap-1">
            <label class="block mb-1 font-medium">Weight</Label>
            <div class="flex">
                <InputText name="weight" type="number" class="flex-1 w-30 mr-1"></InputText>
                <Select name="weightUnits" :options="units" placeholder="Kg" class="w-20 border rounded-r bg-gray-50"/>
            </div>
        </div>
        <div>
            <label class="block mb-1 font-medium">Date</Label>
            <DatePicker name="taken" :show-button-bar="true" showIcon :show-time="true" :placeholder="now.toString()"/>
        </div>
            <Button type="submit" severity="secondary" label="Submit" fluid />
        <div class="pt-2 border-t mt-2 mx-auto">
        <Button 
            label="Detect From Camera"
            icon="pi pi-camera"
            iconPos="left"
            outlined
            class="animate-pulse-slow flex"
            @click="$router.push('/reading-from-camera')"
        />
        </div>

    </Form>
    </div>
    </div>
    </div>
</template>

<style>
/* Gentle custom pulse animation */
@keyframes pulse-slow {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.12); }
}

.animate-pulse-slow .pi {
  animation: pulse-slow 1.8s ease-in-out infinite;
}
</style>
