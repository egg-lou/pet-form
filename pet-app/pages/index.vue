<script setup lang="ts">
import {
    Card,
    CardContent,
    CardHeader,
    CardFooter,
    CardDescription
} from '~/components/ui/card';
const links = ref<{ name: string; href: string; description: string }[]>([
    {
        name: 'Add a new record',
        href: '/registration',
        description: 'Add a new owner and their pets'
    },
    {
        name: 'View records',
        href: '/records',
        description: 'Access all the records for the owners and pets'
    }
]);
</script>

<template>
    <div class="main flex flex-col items-center justify-center">
        <div class="flex items-center gap-4">
            <nuxt-img
                src="/img/icon.png"
                alt="Paws and Claws Vet Clinic"
                width="70"
                height="70" />
            <h1 class="text-4xl font-bold">
                Welcome to Paws and Claws Vet Clinic!
            </h1>
        </div>

        <div class="flex items-center gap-4 py-10">
            <div
                v-for="(dataLink, index) in links"
                :key="index"
                class="flex">
                <Card class="w-96">
                    <CardHeader>{{ dataLink.name }}</CardHeader>
                    <CardContent>
                        <CardDescription>{{
                            dataLink.description
                        }}</CardDescription>
                    </CardContent>
                    <CardFooter>
                        <nuxt-link :to="dataLink.href">
                            <Button variant="outline">Check out</Button>
                        </nuxt-link>
                    </CardFooter>
                </Card>
            </div>
        </div>
    </div>
</template>

<style scoped>
.main {
    min-height: calc(100vh - 4rem - 3rem);
}
</style>
