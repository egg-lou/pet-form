<script setup lang="ts">
import Owner from '~/components/form/owner-form.vue';
</script>

<template>
    <div class="main container flex flex-col items-center justify-center">
        <div>
            <div class="flex items-center gap-4 border-b-4 py-4">
                <nuxt-img
                    src="/img/icon.png"
                    alt="Paws and Claws Vet Clinic"
                    width="60"
                    height="60" />
                <h3 class="text-xl font-bold">Register a new owner and pet</h3>
            </div>
            <div class="w-full">
                <Owner />
            </div>
        </div>
    </div>
</template>
