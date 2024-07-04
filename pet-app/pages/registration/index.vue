<script setup lang="ts">
import OwnerForm from '~/components/form/owner-form.vue';
import PetForm from '~/components/form/pet-form.vue';
import { Switch } from '~/components/ui/switch';
import { ScrollArea } from '~/components/ui/scroll-area';
import { useFormCheckStore } from '~/stores/formCheck';
import { OwnerService } from '~/api/owner';
import { toast } from '~/components/ui/toast';

const ownerService = new OwnerService();
const ownerId = ref(null);
const pets = ref([]);
const ownerInfo = ref({});
const emits = defineEmits(['update:modelValue', 'triggerValidation']);
const formStore = useFormCheckStore();
const addPet = () => {
    pets.value.push({});
};

const removePet = (index) => {
    pets.value.splice(index, 1);
};

const isSwitchOn = ref(false);

const toggleSwitch = () => {
    isSwitchOn.value = !isSwitchOn.value;
};

const onSubmit = () => {
    if (isSwitchOn.value && pets.value.length === 0) {
        toast({
            title: '❌ Error',
            description: 'Please add at least one pet'
        });
        return;
    }
    formStore.validateForm();
};

const handleOwnerAdded = (id) => {
    ownerId.value = id;
};
</script>

<template>
    <div class="main container flex flex-col items-center justify-center">
        <Card
            class="flex h-[85vh] w-full flex-col items-center justify-center gap-3">
            <div class="flex items-center gap-4 py-4">
                <nuxt-img
                    src="/img/icon.png"
                    alt="Paws and Claws Vet Clinic"
                    width="60"
                    height="60" />
                <h3 class="text-xl font-bold">Register a new owner and pet</h3>
            </div>
            <div
                class="grid h-[33rem] w-full grid-cols-2 px-10"
                style="grid-template-columns: 1fr 2fr">
                <div class="px-10">
                    <OwnerForm
                        v-model="ownerInfo"
                        @trigger-validation="onSubmit"
                        @owner-added="handleOwnerAdded" />
                </div>

                <div class="flex flex-col gap-5 px-4">
                    <div class="flex items-center self-end">
                        <div class="flex items-center gap-4">
                            <h3>Include Pet/s</h3>
                            <Switch
                                v-model="isSwitchOn"
                                @click="toggleSwitch" />
                        </div>
                    </div>
                    <div v-if="isSwitchOn">
                        <ScrollArea class="h-[30rem] w-full gap-3 border-2 p-3">
                            <div
                                v-for="(pet, index) in pets"
                                :key="index"
                                class="my-2 flex flex-col border-2 px-4 py-3">
                                <div
                                    class="flex justify-between border-b-2 py-2">
                                    <h3 class="text-md font-semibold">
                                        Pet Number #{{ index + 1 }} Information
                                    </h3>
                                    <Button
                                        variant="destructive"
                                        @click="removePet(index)"
                                        >X</Button
                                    >
                                </div>
                                <PetForm
                                    v-model="pets[index]"
                                    :mode="'add'"
                                    :owner_id="ownerId" />
                            </div>
                            <Button
                                class="my-3 w-full border-2 border-dashed p-10"
                                variant="ghost"
                                @click="addPet">
                                Add Pet
                            </Button>
                        </ScrollArea>
                    </div>
                </div>
            </div>
            <div class="flex w-4/5 flex-col gap-4 py-4">
                <Button variant="outline"> Reset </Button>
                <Button
                    class="dark:text-accent-foreground"
                    @click="onSubmit">
                    Submit
                </Button>
            </div>
        </Card>
    </div>
</template>
