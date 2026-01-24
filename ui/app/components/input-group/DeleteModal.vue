<script setup lang="ts">
import { Trash2 } from 'lucide-vue-next';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '../ui/dialog';
import { Button } from '../ui/button';

const props = defineProps({
    title: String,
    id: String
})

const emit = defineEmits<{
    delete: [id: string]
}>()

</script>

<template>
    <Dialog>
        <DialogTrigger asChild>
            <Button variant="destructive" class="w-full">
                <Trash2 /> Delete This
            </Button>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Delete {{ title }}</DialogTitle>
                <DialogDescription>
                    Are you sure you want to delete this?
                </DialogDescription>
            </DialogHeader>
            <DialogFooter>
                <DialogClose asChild>
                    <Button variant="outline">Cancel</Button>
                </DialogClose>
                <DialogClose asChild>
                    <Button variant="destructive" @click="emit('delete', props.id!)">Confirm</Button>
                </DialogClose>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
