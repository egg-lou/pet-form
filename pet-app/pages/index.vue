<script setup lang="ts">
const links = ref<{ name: string; href: string; description: string }[]>([
    {
        name: 'View Records',
        href: '/records',
        description: 'Access all the records for the owners and pets'
    },
    {
        name: 'View Veterinarians',
        href: '/veterinarians',
        description: 'Access all the veterinarians in the clinic'
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
            <h1 class="text-2xl font-bold">
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
