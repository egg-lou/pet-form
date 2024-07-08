<script setup lang="ts">
import { StatisticService } from '~/api/statistic';

const statisticService = new StatisticService();

const catTotalVisits = ref(0);
const dogTotalVisits = ref(0);
const generalTotal = ref(0);
const surgeryTotal = ref(0);
const groomingTotal = ref(0);
const preventiveTotal = ref(0);
const getCounter = async () => {
    const response = await statisticService.getServicesCounter();
    console.log(response.data.services);

    if (Array.isArray(response.data.services)) {
        response.data.services.forEach(
            (item: { service_type_name: string; total: number }) => {
                if (item.service_type_name === 'General Check-up') {
                    generalTotal.value = item.total;
                } else if (item.service_type_name === 'Surgery') {
                    surgeryTotal.value = item.total;
                } else if (item.service_type_name === 'Grooming') {
                    groomingTotal.value = item.total;
                } else if (item.service_type_name === 'Preventive Care') {
                    preventiveTotal.value = item.total;
                }
            }
        );
    }
};

const data = computed(() => [
    {
        name: 'General Check-up',
        value: generalTotal.value,
        image: '/img/General.png'
    },
    {
        name: 'Surgery',
        value: surgeryTotal.value,
        image: '/img/Surgery.png'
    },
    {
        name: 'Grooming',
        value: groomingTotal.value,
        image: '/img/Grooming.png'
    },
    {
        name: 'Preventive Care',
        value: preventiveTotal.value,
        image: '/img/Preventive.png'
    }
]);

const updateTotalVisits = async () => {
    const response = await statisticService.getServicesPetTypeSummary();

    if (Array.isArray(response.data.pet_type_visit_summary)) {
        response.data.pet_type_visit_summary.forEach(
            (item: { pet_type: string; total_visits: number }) => {
                if (item.pet_type === 'Dog') {
                    dogTotalVisits.value = item.total_visits;
                } else if (item.pet_type === 'Cat') {
                    catTotalVisits.value = item.total_visits;
                }
            }
        );
    }
};

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

onMounted(() => {
    getCounter();
    updateTotalVisits();
});
</script>

<template>
    <div class="main flex flex-col items-center justify-center">
        <div class="flex w-full items-center justify-evenly">
            <div class="flex flex-col items-center">
                <nuxt-img
                    src="/img/Dog.png"
                    alt="Dog Logo"
                    width="100"
                    height="100" />
                <h3 class="text-md font-semibold">
                    Total Visits: {{ dogTotalVisits }}
                </h3>
            </div>
            <div class="flex flex-col items-center">
                <nuxt-img
                    src="/img/Cat.png"
                    alt="Cat Logo"
                    width="100"
                    height="100" />
                <h3 class="text-md font-semibold">
                    Total Visits: {{ dogTotalVisits }}
                </h3>
            </div>
        </div>
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
        <div class="flex gap-10">
            <div
                v-for="item in data"
                :key="item.name"
                class="flex flex-col items-center gap-3 px-3">
                <nuxt-img
                    :src="item.image"
                    :alt="item.name"
                    width="50"
                    height="50" />
                <h3 class="text-md font-semibold">
                    {{ item.name }}: {{ item.value }}
                </h3>
            </div>
        </div>
    </div>
</template>
