<script setup lang="ts">
import {
    preventiveCareTypesCat,
    preventiveCareTypesDog
} from '~/utils/checkBoxItems';
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';
import * as z from 'zod';
import { ServiceInstanceService } from '~/api/service-instance';
import { VetService } from '~/api/vet';
import type { VetList } from '~/types/vet-type';
import { DialogClose } from 'radix-vue';
import { toast } from '~/components/ui/toast';
const vetService = new VetService();
const { service_instance_id } = useRoute().params;
const serviceInstanceService = new ServiceInstanceService();

const props = defineProps({
    pet_type: String
});
const vetLists = ref<VetList[]>([]);
const isDialogOpen = ref(false);

const getVets = async () => {
    const response = await vetService.vetLists();
    vetLists.value = response.data.vets;
};

const addPreventiveSchema = toTypedSchema(
    z.object({
        treatment: z
            .array(z.string())
            .nonempty()
            .refine((value) => value !== null, {
                message: 'Please select a treatment'
            }),
        vet_id: z
            .string()
            .nonempty()
            .refine((value) => value !== null, {
                message: 'Please select a veterinarian'
            }),
        service_instance_id: z.string()
    })
);

const { handleSubmit } = useForm({
    validationSchema: addPreventiveSchema,
    initialValues: {
        treatment: [],
        vet_id: '',
        service_instance_id: service_instance_id
    }
});

const onSubmit = handleSubmit(async (values) => {
    await serviceInstanceService
        .addPreventiveCare(service_instance_id, values)
        .then(() => {
            toast({
                title: '✅ Preventive Care added successfully',
                description: 'Preventive Care has been added successfully'
            });
            isDialogOpen.value = false;
            const fetch = useRefetchStore();
            fetch.triggerRefetch();
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description:
                    'An error occurred while adding preventive care.' +
                    error.message
            });
        });
});

const resetForm = () => {};
</script>
<template>
    <Dialog v-model:open="isDialogOpen">
        <DialogTrigger
            class="w-full border-2 border-dashed p-2"
            @click="getVets">
            <p>Add</p>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>Add Preventive Care: </DialogHeader>
            <DialogDescription>
                <FormField name="treatment">
                    <FormItem class="flex flex-col gap-2">
                        <div class="types-container">
                            <FormField
                                v-for="item in props.pet_type === 'Cat'
                                    ? preventiveCareTypesCat
                                    : preventiveCareTypesDog"
                                v-slot="{ value, handleChange }"
                                :key="item.id"
                                type="checkbox"
                                :value="item.id"
                                :unchecked-value="false"
                                name="treatment">
                                <FormItem
                                    class="flex flex-row items-start space-x-3 space-y-0">
                                    <FormControl>
                                        <Checkbox
                                            :checked="value.includes(item.id)"
                                            @update:checked="handleChange" />
                                    </FormControl>
                                    <FormLabel>
                                        {{ item.label }}
                                    </FormLabel>
                                </FormItem>
                            </FormField>
                        </div>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <FormField
                    v-slot="{ componentField }"
                    name="vet_id">
                    <FormItem class="flex w-full items-center gap-3 space-y-2">
                        <FormLabel>Veterinarian: </FormLabel>
                        <Select v-bind="componentField">
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
                <div class="flex justify-end gap-4 self-end py-4">
                    <Button
                        type="reset"
                        class="self-end"
                        variant="destructive"
                        @click="resetForm">
                        Reset
                    </Button>
                    <Button
                        type="submit"
                        class="self-end dark:text-accent-foreground"
                        @click="onSubmit">
                        Submit
                    </Button>
                </div>
            </DialogDescription>
        </DialogContent>
    </Dialog>
</template>
<style scoped>
.types-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 20px;
}
</style>
