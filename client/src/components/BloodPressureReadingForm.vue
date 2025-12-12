<script setup>

function onSubmit(param) {
    console.log(param)
}

</script>

<template>
      <div class="max-w-sm mx-auto">
    <div class="bg-white rounded-2xl shadow p-6 border">
    <Form v-slot="$form" @submit="onSubmit"  class="flex flex-col gap-4 w-full sm:w-56 mx-auto">
        <div class="flex flex-col gap-1">
        <Label class="block mb-1 font-medium">Systolic</Label>
            <InputText name="systolic" type="text" placeholder="" fluid></InputText>
        </div>
        <div class="flex flex-col gap-1">
        <Label class="block mb-1 font-medium">Diastolic</Label>
            <InputText name="diastolic" type="text" placeholder="" fluid></InputText>
        </div>
        <div class="flex flex-col gap-1">
        <Label class="block mb-1 font-medium">Diastolic</Label>
            <InputText name="pulse" type="text" placeholder="" fluid></InputText>
        </div>
            <Button type="submit" severity="secondary" label="Submit" />
        <div class="pt-2 border-t mt-2">
        <Button 
        label="Detect From Camera"
        icon="pi pi-camera"
        iconPos="left"
        outlined
        class="animate-pulse-slow"
        />
        </div>

    </Form>
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
