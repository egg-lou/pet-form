<script setup lang="ts">
import { PenIcon } from 'lucide-vue-next';

import { ServiceInstanceService } from '~/api/service-instance';
import { VetService } from '~/api/vet';
import { toTypedSchema } from '@vee-validate/zod';
import * as z from 'zod';
import { useForm } from 'vee-validate';
import { Textarea } from '~/components/ui/textarea';
import type { VetList } from '~/types/vet-type';
import { DialogClose } from 'radix-vue';
import { toast } from '~/components/ui/toast';
const { service_instance_id } = useRoute().params;

const serviceInstanceService = new ServiceInstanceService();
const vetService = new VetService();

const fetchServiceInstance = async () => {
    const response =
        await serviceInstanceService.getServiceInstance(service_instance_id);
    setValues({
        surgery_name: response.data.surgery[0].surgery_name,
        veterinarian_diagnosis: response.data.surgery[0].veterinarian_diagnosis,
        anesthesia_used: response.data.surgery[0].anesthesia_used,
        complications: response.data.surgery[0].complications,
        outcome: response.data.surgery[0].outcome,
        vet_id: response.data.surgery[0].vet.vet_id
    });
    surgeryId.value = response.data.surgery[0].surgery_id;
    selectedVetId.value = response.data.surgery[0].vet.vet_id;
    await getVets();
};
const vetLists = ref<VetList[]>([]);
const selectedVetId = ref('');
const surgeryId = ref(0);

const getVets = async () => {
    const response = await vetService.vetLists();
    vetLists.value = response.data.vets;
};
const updateSurgery = toTypedSchema(
    z.object({
        surgery_name: z.string().nonempty(),
        veterinarian_diagnosis: z.string().nonempty(),
        anesthesia_used: z.string().nonempty(),
        complications: z.string().nonempty(),
        outcome: z.string().nonempty(),
        vet_id: z.string().nonempty()
    })
);

const { handleSubmit, isFieldDirty, values, setFieldValue, setValues } =
    useForm({
        validationSchema: updateSurgery
    });

const onSubmit = handleSubmit(async (values) => {
    await serviceInstanceService
        .updateSurgery(surgeryId.value, values)
        .then(() => {
            toast({
                title: '✅ Surgery updated successfully',
                description: 'Surgery has been updated successfully'
            });
            const fetch = useRefetchStore();
            fetch.triggerRefetch();
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description: error.response.data.message
            });
        });
});

onMounted(async () => {
    await getVets();
    await fetchServiceInstance();
});
</script>

<template>
    <Dialog>
        <DialogTrigger>
            <Button
                variant="ghost"
                @click="fetchServiceInstance">
                <PenIcon class="h-5 w-5" />
            </Button>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>Edit Surgery: </DialogHeader>
            <DialogDescription>
                <form
                    class="flex flex-col gap-5"
                    @submit.prevent="onSubmit">
                    <FormField
                        v-slot="{ componentField }"
                        name="surgery_name"
                        :validate-on-blur="!isFieldDirty('surgery_name')">
                        <FormItem class="flex w-full items-center">
                            <FormLabel>Surgery Name: </FormLabel>
                            <FormControl>
                                <Input
                                    type="text"
                                    placeholder="Surgery Name"
                                    v-bind="componentField" />
                            </FormControl>
                        </FormItem>
                    </FormField>

                    <FormField
                        v-slot="{ componentField }"
                        name="anesthesia_used"
                        :validate-on-blur="!isFieldDirty('anesthesia_used')">
                        <FormItem class="flex w-full items-center">
                            <FormLabel>Anesthesia Used: </FormLabel>
                            <FormControl>
                                <Input
                                    type="text"
                                    placeholder="Anesthesia Used"
                                    v-bind="componentField" />
                            </FormControl>
                        </FormItem>
                    </FormField>
                    <div class="flex items-center justify-evenly gap-5">
                        <FormField
                            v-slot="{ componentField }"
                            name="veterinarian_diagnosis"
                            :validate-on-blur="
                                !isFieldDirty('veterinarian_diagnosis')
                            ">
                            <FormItem class="w-full">
                                <FormLabel>Veterinarian Diagnosis: </FormLabel>
                                <FormControl>
                                    <Textarea
                                        placeholder="Veterinarian Diagnosis"
                                        class="h-[8rem] resize-none"
                                        v-bind="componentField" />
                                </FormControl>
                            </FormItem>
                        </FormField>
                        <FormField
                            v-slot="{ componentField }"
                            name="complications"
                            :validate-on-blur="!isFieldDirty('complications')">
                            <FormItem class="w-full">
                                <FormLabel>Complications:</FormLabel>
                                <FormControl>
                                    <Textarea
                                        placeholder="Complications"
                                        class="h-[8rem] resize-none"
                                        v-bind="componentField" />
                                </FormControl>
                            </FormItem>
                        </FormField>
                    </div>
                    <FormField
                        v-slot="{ componentField }"
                        name="outcome">
                        <FormItem class="flex w-full items-center gap-3">
                            <FormLabel>Outcome: </FormLabel>
                            <FormControl>
                                <Input
                                    type="text"
                                    placeholder="Outcome"
                                    v-bind="componentField" />
                            </FormControl>
                        </FormItem>
                    </FormField>
                    <FormField
                        v-slot="{ componentField }"
                        name="vet_id">
                        <FormItem
                            class="flex w-full items-center gap-3 space-y-2">
                            <FormLabel>Veterinarian: </FormLabel>
                            <Select
                                v-bind="componentField"
                                v-model="selectedVetId">
                                <FormControl>
                                    <SelectTrigger>
                                        <SelectValue
                                            placeholder="Select a veterinarian" />
                                    </SelectTrigger>
                                </FormControl>
                                <SelectContent>
                                    <SelectGroup
                                        v-for="vet in vetLists"
                                        :key="vet.vet_id">
                                        <SelectItem :value="vet.vet_id">{{
                                            vet.vet_name
                                        }}</SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </FormItem>
                    </FormField>

                    <div class="flex gap-4 self-end py-4">
                        <DialogClose as-child>
                            <Button
                                type="submit"
                                class="self-end dark:text-accent-foreground"
                                @click="onSubmit">
                                Submit
                            </Button>
                        </DialogClose>
                    </div>
                </form>
            </DialogDescription>
        </DialogContent>
    </Dialog>
</template>

<style scoped></style>
