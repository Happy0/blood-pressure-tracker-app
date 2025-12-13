<script setup>

    const now = new Date()

    const props = defineProps({
        systolic: Number,
        diastolic: Number,
        pulse: Number
        }
    )

    const initial = {
        systolic: props.systolic,
        diastolic: props.diastolic,
        pulse: props.pulse,
        dateTime: now
    }

    function onSubmit(param) {
        console.log(param)
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
        <div>
        <label class="block mb-1 font-medium">Date</Label>
        <DatePicker name="dateTime" :show-button-bar="true" showIcon :show-time="true" :placeholder="now.toString()"/>
        </div>
            <Button type="submit" severity="secondary" label="Submit" fluid />
        <div class="pt-2 border-t mt-2 mx-auto">
        <Button 
        label="Detect From Camera"
        icon="pi pi-camera"
        iconPos="left"
        outlined
        class="animate-pulse-slow flex"
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
