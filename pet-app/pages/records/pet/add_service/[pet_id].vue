<script setup lang="ts">
import {
    serviceTypes,
    groomingTypes,
    preventiveCareTypesCat,
    preventiveCareTypesDog
} from '~/utils/checkBoxItems';
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';
import * as z from 'zod';
import { Textarea } from '~/components/ui/textarea';
import { CalendarIcon } from '@radix-icons/vue';
import { cn } from '~/lib/utils';
import {
    CalendarDate,
    DateFormatter,
    parseDate
} from '@internationalized/date';
import { toDate } from 'radix-vue/date';
import { VetService } from '~/api/vet';
import { PetService } from '~/api/pet';
import { ServiceInstanceService } from '~/api/service-instance';
import type { VetList } from '~/types/vet-type';
import { toast } from '~/components/ui/toast';
import { useRouter } from 'vue-router';
const { pet_id } = useRoute().params;
const pet_type = ref('');
const df = new DateFormatter('en-US', {
    dateStyle: 'long'
});
const petService = new PetService();
const vetService = new VetService();
const serviceInstanceService = new ServiceInstanceService();
const placeholder = ref();
const router = useRouter();
const vetLists = ref<VetList[]>([]);

const fetchPet = async () => {
    const response = await petService.getPet(pet_id);
    pet_type.value = response.data.pet.pet_type;
};
const getVets = async () => {
    const response = await vetService.vetLists();
    vetLists.value = response.data.vets;
};
const value = computed({
    get: () =>
        values.followup_date ? parseDate(values.followup_date) : undefined,
    set: (val) => val
});
const formSchema = toTypedSchema(
    z.object({
        pet_id: z.string(),
        service_reason: z.string(),
        general_diagnosis: z.string(),
        requires_followup: z.boolean(),
        followup_date: z.string().nullable().optional(),
        service_type: z
            .array(z.string())
            .refine((value) => value.some((item) => item), {
                message: 'You have to select at least one item.'
            }),
        grooming_type: z
            .array(z.string())
            .optional()
            .refine((value) => value !== null, {
                message: 'Grooming type cannot be null.'
            }),
        preventive_care: z
            .object({
                treatment: z
                    .array(z.string())
                    .optional()
                    .refine((value) => value !== null, {
                        message: 'Treatment cannot be null.'
                    }),
                vet_id: z.string()
            })
            .optional(),
        surgery: z
            .object({
                surgery_name: z.string(),
                anesthesia_used: z.string(),
                veterinarian_diagnosis: z.string(),
                complications: z.string(),
                outcome: z.string(),
                vet_id: z.string()
            })
            .optional()
    })
);
const { handleSubmit, isFieldDirty, values, setFieldValue } = useForm({
    validationSchema: formSchema,
    initialValues: {
        pet_id: Array.isArray(pet_id) ? pet_id[0] : pet_id,
        service_type: ['General Check-up'],
        grooming_type: [],
        requires_followup: false,
        preventive_care: {
            treatment: [],
            vet_id: ''
        }
    }
});
const isGroomingSelected = computed(() =>
    values.service_type.includes('Grooming')
);
const isPreventiveCareSelected = computed(() =>
    values.service_type.includes('Preventive Care')
);
const isSurgerySelected = computed(() =>
    values.service_type.includes('Surgery')
);
const onSubmit = handleSubmit(async (values) => {
    if (
        values.preventive_care &&
        values.preventive_care.treatment.length === 0
    ) {
        values.preventive_care = null;
    }

    if (values.grooming_type.length === 0) {
        values.grooming_type = null;
    }
    const surgeryFields = values.surgery ? Object.values(values.surgery) : [];
    if (surgeryFields.every((field) => field === '' || field === undefined)) {
        values.surgery = null;
    }

    await serviceInstanceService
        .addServiceInstance(values)
        .then((response) => {
            toast({
                title: '✅ Service Record Added',
                description: 'Service record has been added successfully'
            });
            router.push(
                `/records/pet/service_instance/${response.data.service_instance_id}`
            );
        })
        .catch((error) => {
            toast({
                title: 'Service Record Failed',
                description: '❌ Failed to add service record' + error.message
            });
        });
});

onMounted(() => {
    fetchPet();
    getVets();
});
</script>

<template>
    <div class="main px-20">
        <Card>
            <div class="flex h-[89vh] flex-col justify-center p-10">
                <div class="flex">
                    <div class="flex w-1/2">
                        <h1 class="text-lg font-bold">
                            Add Service Record for {{ pet_type }}
                        </h1>
                    </div>
                    <div class="w-1/2">
                        <div class="flex justify-end">
                            <Button
                                class="dark:text-accent-foreground"
                                type="submit"
                                @click="onSubmit"
                                >Save</Button
                            >
                        </div>
                    </div>
                </div>
                <form
                    class="grid grid-cols-2 gap-4 px-2 py-3"
                    style="grid-template-columns: 1fr 2fr"
                    @submit.prevent="onSubmit">
                    <div class="flex flex-col gap-5 border-2 p-3">
                        <FormField
                            name="pet_id"
                            :validate-on-blur="!isFieldDirty('pet_id')">
                            <FormItem>
                                <FormLabel>Pet ID</FormLabel>
                                <FormControl>
                                    <Input
                                        type="text"
                                        v-model="pet_id"
                                        disabled />
                                </FormControl>
                            </FormItem>
                        </FormField>
                        <FormField name="service_type">
                            <FormItem>
                                <FormLabel> Service Type: </FormLabel>
                                <FormField
                                    v-for="item in serviceTypes"
                                    v-slot="{ value, handleChange }"
                                    :key="item.id"
                                    v-model="values.service_type"
                                    type="checkbox"
                                    :value="item.id"
                                    :unchecked-value="false"
                                    name="service_type">
                                    <FormItem
                                        class="flex flex-row items-start space-x-3 space-y-0">
                                        <FormControl>
                                            <Checkbox
                                                :checked="
                                                    value.includes(item.id)
                                                "
                                                @update:checked="
                                                    handleChange
                                                " />
                                        </FormControl>
                                        <FormLabel>
                                            {{ item.label }}
                                        </FormLabel>
                                    </FormItem>
                                </FormField>
                                <FormMessage />
                            </FormItem>
                        </FormField>
                        <FormField
                            v-slot="{ componentField }"
                            name="service_reason"
                            :validate-on-blur="!isFieldDirty('service_reason')">
                            <FormItem class="w-full">
                                <FormLabel>Service Reason: </FormLabel>
                                <FormControl>
                                    <Textarea
                                        placeholder="Service Reason"
                                        class="resize-none"
                                        v-bind="componentField" />
                                </FormControl>
                            </FormItem>
                        </FormField>

                        <FormField
                            v-slot="{ componentField }"
                            name="general_diagnosis"
                            :validate-on-blur="
                                !isFieldDirty('general_diagnosis')
                            ">
                            <FormItem class="w-full">
                                <FormLabel>General Diagnosis: </FormLabel>
                                <FormControl>
                                    <Textarea
                                        placeholder="General Diagnosis"
                                        class="resize-none"
                                        v-bind="componentField" />
                                </FormControl>
                            </FormItem>
                        </FormField>

                        <FormField
                            v-slot="{ value, handleChange }"
                            name="requires_followup"
                            :validate-on-blur="
                                !isFieldDirty('requires_followup')
                            ">
                            <FormItem
                                class="flex w-full items-center gap-3 space-y-0">
                                <FormLabel>Requires Follow Up: </FormLabel>
                                <FormControl>
                                    <Switch
                                        v-model="values.requires_followup"
                                        :checked="value"
                                        @update:checked="handleChange" />
                                </FormControl>
                            </FormItem>
                        </FormField>

                        <FormField
                            v-if="values.requires_followup"
                            name="followup_date">
                            <FormItem
                                class="flex items-center space-x-3 space-y-0">
                                <FormLabel>Follow Up Date: </FormLabel>
                                <Popover>
                                    <PopoverTrigger as-child>
                                        <FormControl>
                                            <Button
                                                variant="outline"
                                                :class="
                                                    cn(
                                                        'w-[240px] ps-3 text-start font-normal',
                                                        !value &&
                                                            'text-muted-foreground'
                                                    )
                                                ">
                                                <span>{{
                                                    value
                                                        ? df.format(
                                                              toDate(value)
                                                          )
                                                        : 'Pick a date'
                                                }}</span>
                                                <CalendarIcon
                                                    class="ms-auto h-4 w-4 opacity-50" />
                                            </Button>
                                            <input hidden />
                                        </FormControl>
                                    </PopoverTrigger>
                                    <PopoverContent class="w-auto p-0">
                                        <Calendar
                                            v-model:placeholder="placeholder"
                                            v-model="value"
                                            calendar-label="Date of birth"
                                            initial-focus
                                            :min-value="
                                                new CalendarDate(1900, 1, 1)
                                            "
                                            @update:model-value="
                                                (v) => {
                                                    if (v) {
                                                        setFieldValue(
                                                            'followup_date',
                                                            v.toString()
                                                        );
                                                    } else {
                                                        setFieldValue(
                                                            'followup_date',
                                                            undefined
                                                        );
                                                    }
                                                }
                                            " />
                                    </PopoverContent>
                                </Popover>
                            </FormItem>
                        </FormField>
                    </div>
                    <div class="flex flex-col justify-center gap-4">
                        <div class="flex h-auto w-full gap-4">
                            <div
                                v-if="isGroomingSelected"
                                class="w-1/2 border-2 p-5">
                                <FormField name="grooming_type">
                                    <FormItem class="flex flex-col gap-3">
                                        <FormLabel> Grooming: </FormLabel>
                                        <div class="types-container">
                                            <FormField
                                                v-for="item in groomingTypes"
                                                v-slot="{ value, handleChange }"
                                                :key="item.id"
                                                type="checkbox"
                                                :value="item.id"
                                                :unchecked-value="false"
                                                name="grooming_type">
                                                <FormItem
                                                    class="flex flex-row items-start space-x-3 space-y-0">
                                                    <FormControl>
                                                        <Checkbox
                                                            :checked="
                                                                value.includes(
                                                                    item.id
                                                                )
                                                            "
                                                            @update:checked="
                                                                handleChange
                                                            " />
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
                            </div>
                            <div
                                v-if="isPreventiveCareSelected"
                                class="w-1/2 border-2 p-4">
                                <FormField name="treatment">
                                    <FormItem class="flex flex-col gap-2">
                                        <FormLabel>
                                            Preventive Care:
                                        </FormLabel>
                                        <div class="flex flex-col gap-4">
                                            <div class="types-container">
                                                <FormField
                                                    v-for="item in pet_type ===
                                                    'Cat'
                                                        ? preventiveCareTypesCat
                                                        : preventiveCareTypesDog"
                                                    v-slot="{
                                                        value,
                                                        handleChange
                                                    }"
                                                    :key="item.id"
                                                    type="checkbox"
                                                    :value="item.id"
                                                    :unchecked-value="false"
                                                    name="preventive_care.treatment">
                                                    <FormItem
                                                        class="flex flex-row items-start space-x-3 space-y-0">
                                                        <FormControl>
                                                            <Checkbox
                                                                :checked="
                                                                    value.includes(
                                                                        item.id
                                                                    )
                                                                "
                                                                @update:checked="
                                                                    handleChange
                                                                " />
                                                        </FormControl>
                                                        <FormLabel>
                                                            {{ item.label }}
                                                        </FormLabel>
                                                    </FormItem>
                                                </FormField>
                                            </div>
                                            <FormField
                                                v-slot="{ componentField }"
                                                name="preventive_care.vet_id">
                                                <FormItem
                                                    class="flex w-full items-center gap-3 space-y-2">
                                                    <FormLabel
                                                        >Veterinarian:
                                                    </FormLabel>
                                                    <Select
                                                        v-bind="componentField">
                                                        <FormControl>
                                                            <SelectTrigger>
                                                                <SelectValue
                                                                    placeholder="Select a veterinarian" />
                                                            </SelectTrigger>
                                                        </FormControl>
                                                        <SelectContent>
                                                            <SelectGroup
                                                                v-for="vet in vetLists"
                                                                :key="
                                                                    vet.vet_id
                                                                ">
                                                                <SelectItem
                                                                    :value="
                                                                        vet.vet_id
                                                                    "
                                                                    >{{
                                                                        vet.vet_name
                                                                    }}</SelectItem
                                                                >
                                                            </SelectGroup>
                                                        </SelectContent>
                                                    </Select>
                                                </FormItem>
                                            </FormField>
                                        </div>
                                        <FormMessage />
                                    </FormItem>
                                </FormField>
                            </div>
                        </div>
                        <div
                            v-if="isSurgerySelected"
                            class="flex flex-col gap-3 border-2 p-2">
                            <FormField
                                v-slot="{ componentField }"
                                name="surgery">
                                <FormLabel>Surgery:</FormLabel>
                            </FormField>
                            <FormField
                                v-slot="{ componentField }"
                                name="surgery.surgery_name"
                                :validate-on-blur="
                                    !isFieldDirty('surgery.surgery_name')
                                ">
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
                                name="surgery.anesthesia_used"
                                :validate-on-blur="
                                    !isFieldDirty('surgery.anesthesia_used')
                                ">
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
                                    name="surgery.veterinarian_diagnosis"
                                    :validate-on-blur="
                                        !isFieldDirty(
                                            'surgery.veterinarian_diagnosis'
                                        )
                                    ">
                                    <FormItem class="w-full">
                                        <FormLabel
                                            >Veterinarian Diagnosis:
                                        </FormLabel>
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
                                    name="surgery.complications"
                                    :validate-on-blur="
                                        !isFieldDirty('surgery.complications')
                                    ">
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
                                name="surgery.outcome">
                                <FormItem
                                    class="flex w-full items-center gap-3">
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
                                name="surgery.vet_id">
                                <FormItem
                                    class="flex w-full items-center gap-3 space-y-2">
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
                                                <SelectItem
                                                    :value="vet.vet_id"
                                                    >{{
                                                        vet.vet_name
                                                    }}</SelectItem
                                                >
                                            </SelectGroup>
                                        </SelectContent>
                                    </Select>
                                </FormItem>
                            </FormField>
                        </div>
                    </div>
                </form>
            </div>
        </Card>
    </div>
</template>

<style scoped>
.types-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 20px;
}
</style>
