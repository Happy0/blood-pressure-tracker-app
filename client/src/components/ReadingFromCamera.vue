<script lang="ts" setup>
    import { onMounted, onUnmounted } from 'vue';
    import { useRouter } from 'vue-router';

    const router = useRouter()

    type BloodPressureReading = {
        systolic: number,
        diastolic: number,
        pulse: number
    }

    let intervalId: number | undefined = undefined;

    onUnmounted(() => {
        console.log("Clearing! " + intervalId);
        clearInterval(intervalId)
    })

    onMounted(() => {
        getValueFromVideoStream().then(bloodPressureReading => {
            router.push({
                path: `/reading-with-values/systolic/${bloodPressureReading.systolic}/diastolic/${bloodPressureReading.diastolic}/pulse/${bloodPressureReading.pulse}`
            })
        })
    })

    function getValueFromVideoStream(): Promise<BloodPressureReading> {
        // TODO: make this less spaghetti like with the callbacks, etc

        return new Promise((resolve, reject) => {
            navigator.mediaDevices
                .getUserMedia({video: {facingMode: 'environment'}, audio: false})
                .then((localMediaStream) => {
                    const video = document.querySelector("video");

                    if (video === null) {
                        return;
                    }

                    video.srcObject = localMediaStream;

                    const mediaStream = localMediaStream.getVideoTracks()[0]

                    if (!mediaStream) {
                        return
                    }

                    intervalId = setInterval(() => {
                        // Cast to any as compiler doesn't think 'grabFrame' exists for some reason
                        const imageCapture = new ImageCapture(mediaStream) as any;

                        imageCapture.grabFrame()
                            //.then(r => r.arrayBuffer())
                            .then(async (imageBitmap: any) => {
                                const ocanvas = new OffscreenCanvas(imageBitmap.width, imageBitmap.height);
                                const renderer = ocanvas.getContext('bitmaprenderer')

                                if (!renderer) {
                                    return
                                }
                                
                                renderer.transferFromImageBitmap(imageBitmap);
                                const blob = await ocanvas.convertToBlob({ type: 'image/png' });

                            const formData = new FormData();
                            formData.append("image", blob, "testaroonie.png")

                            console.log(formData)

                            fetch("/api/run-ocr", {
                                method: 'POST',
                                body: formData
                            })
                            .then(resp => {
                                const result = resp.json().then(respJson => {
                                    if (respJson.type === "Reading") {
                                        clearInterval(intervalId)
                                        resolve(respJson)
                                    }
                                })
                            })
                        })
                    }, 1000)
            }).catch(e => reject(e))
        })
    }

</script>

<template>
    <div>
        <video autoplay></video>
        <canvas></canvas>
    </div>

</template>

<style>

</style>