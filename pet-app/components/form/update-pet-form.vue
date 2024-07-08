<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod';
import * as z from 'zod';
import { useForm } from 'vee-validate';
import { toast } from '~/components/ui/toast';
import { DateFormatter, parseDate } from '@internationalized/date';
import { RadioGroup, RadioGroupItem } from '~/components/ui/radio-group';
import {
    Popover,
    PopoverContent,
    PopoverTrigger
} from '~/components/ui/popover';
import { CalendarIcon } from 'lucide-vue-next';
import { cn } from '~/lib/utils';
import { toDate } from 'radix-vue/date';
import { PetService } from '~/api/pet';
import { DialogClose } from 'radix-vue';
import DateCalendar from '~/components/calendars/date-calendar.vue';
import type { Pet } from '~/types/pet-type';
const { pet_id } = useRoute().params;

const df = new DateFormatter('en-US', {
    dateStyle: 'long'
});

const pet = ref<Pet | null>(null);

const petSchema = toTypedSchema(
    z.object({
        owner_id: z.string().max(36).optional(),
        pet_name: z.string().max(80),
        pet_type: z.enum(['Dog', 'Cat'], {
            required_error: 'Please select a pet type.'
        }),
        pet_breed: z.string().max(40),
        pet_weight: z.number().min(0),
        pet_birth_date: z
            .string()
            .refine((v) => v, { message: 'A date of birth is required.' }),
        pet_color: z.string().max(20)
    })
);

const petService = new PetService();
const value = computed({
    get: () =>
        values.pet_birth_date ? parseDate(values.pet_birth_date) : undefined,
    set: (val) => val
});

const { isFieldDirty, handleSubmit, values, setFieldValue, setValues } =
    useForm({
        validationSchema: petSchema
    });

const fetchPet = async () => {
    const response = await petService.getPet(pet_id);
    const updatedPetData = {
        ...response.data.pet,
        pet_weight: Number(response.data.pet.pet_weight)
    };
    setValues(updatedPetData);
};

const onSubmit = handleSubmit((values) => {
    petService
        .updatePet(values, pet_id)
        .then(() => {
            toast({
                title: '✅ Pet updated successfully',
                description: 'Pet has been updated successfully'
            });
            const fetch = useRefetchStore();
            fetch.triggerRefetch();
        })
        .catch((error) => {
            toast({
                title: '❌ Error',
                description: 'Failed to update pet' + error.message
            });
        });
});

const handleDateChange = (newDate) => {
    setFieldValue('pet_birth_date', newDate);
};

const resetForm = () => {
    setFieldValue('pet_name', '');
    setFieldValue('pet_breed', '');
    setFieldValue('pet_weight', 0);
    setFieldValue('pet_birth_date', '');
    setFieldValue('pet_color', '');
    setFieldValue('owner_id', '');
};
</script>

<template>
    <Dialog>
        <DialogTrigger>
            <Button
                class="dark:text-accent-foreground"
                @click="fetchPet"
                >Update Pet</Button
            >
        </DialogTrigger>
        <DialogContent>
            <h3 class="text-lg font-semibold">Pet Information</h3>
            <form
                class="flex w-full flex-col space-y-6 py-4"
                @submit="onSubmit">
                <FormField
                    v-slot="{ componentField }"
                    name="pet_id"
                    :validate-on-blur="!isFieldDirty('pet_id')">
                    <FormItem class="w-full">
                        <FormLabel>ID: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Pet ID"
                                v-bind="componentField"
                                disabled />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <div class="flex gap-4">
                    <FormField
                        v-slot="{ componentField }"
                        name="pet_name"
                        :validate-on-blur="!isFieldDirty('pet_name')">
                        <FormItem class="w-full">
                            <FormLabel>Name: </FormLabel>
                            <FormControl>
                                <Input
                                    type="text"
                                    placeholder="Pet Name"
                                    v-bind="componentField" />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    </FormField>

                    <FormField
                        v-slot="{ componentField }"
                        name="pet_weight"
                        :validate-on-blur="!isFieldDirty('pet_weight')">
                        <FormItem class="w-full">
                            <FormLabel>Weight: (kg) </FormLabel>
                            <FormControl>
                                <Input
                                    type="number"
                                    placeholder="Pet Weight"
                                    v-bind="componentField" />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    </FormField>
                </div>
                <FormField
                    name="pet_birth_date"
                    :validate-on-blur="!isFieldDirty('pet_birth_date')">
                    <FormItem class="just flex w-full items-center gap-3">
                        <FormLabel>Birth Date: </FormLabel>
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
                                                ? df.format(toDate(value))
                                                : 'Pick a date'
                                        }}</span>
                                        <CalendarIcon
                                            class="ms-auto h-4 w-4 opacity-50" />
                                    </Button>
                                    <input hidden />
                                </FormControl>
                            </PopoverTrigger>
                            <PopoverContent>
                                <DateCalendar @update:date="handleDateChange" />
                            </PopoverContent>
                        </Popover>
                        <FormMessage />
                    </FormItem>
                </FormField>

                <FormField
                    v-slot="{ componentField }"
                    name="pet_color"
                    :validate-on-blur="!isFieldDirty('pet_color')">
                    <FormItem class="w-full">
                        <FormLabel>Color: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Pet Color"
                                v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>

                <FormField
                    v-slot="{ componentField }"
                    type="radio"
                    name="pet_type">
                    <FormItem class="space-y-3">
                        <FormControl>
                            <RadioGroup
                                class="flex items-center gap-4 space-y-1"
                                v-bind="componentField">
                                <FormLabel>Type: </FormLabel>
                                <FormItem
                                    class="flex items-center gap-x-3 space-y-0">
                                    <FormControl>
                                        <RadioGroupItem value="Dog" />
                                    </FormControl>
                                    <FormLabel class="font-normal">
                                        Dog
                                    </FormLabel>
                                </FormItem>
                                <FormItem
                                    class="flex items-center gap-x-3 space-y-0">
                                    <FormControl>
                                        <RadioGroupItem value="Cat" />
                                    </FormControl>
                                    <FormLabel class="font-normal">
                                        Cat
                                    </FormLabel>
                                </FormItem>
                            </RadioGroup>
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <FormField
                    v-slot="{ componentField }"
                    name="pet_breed"
                    :validate-on-blur="!isFieldDirty('pet_breed')">
                    <FormItem class="w-full">
                        <FormLabel>Breed: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Pet Breed"
                                v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <FormField
                    v-slot="{ componentField }"
                    name="owner_id"
                    :validate-on-blur="!isFieldDirty('owner_id')">
                    <FormItem class="w-full">
                        <FormLabel>Owner ID: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Pet ID"
                                v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <FormField
                    v-slot="{ componentField }"
                    name="owner_name"
                    :validate-on-blur="!isFieldDirty('owner_name')">
                    <FormItem class="w-full">
                        <FormLabel>Owner Name: </FormLabel>
                        <FormControl>
                            <Input
                                type="text"
                                placeholder="Pet ID"
                                v-bind="componentField"
                                disabled />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>

                <div class="flex gap-4 self-end py-4">
                    <Button
                        type="reset"
                        class="self-end"
                        variant="destructive"
                        @click="resetForm">
                        Reset
                    </Button>
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
        </DialogContent>
    </Dialog>
</template>

<style scoped></style>
