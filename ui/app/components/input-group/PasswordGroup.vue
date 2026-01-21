<script setup lang="ts">
import { Check, CircleCheck, Copy, Globe, KeyRound, LockKeyhole, Mail, RefreshCw, X } from 'lucide-vue-next';
import { InputGroup, InputGroupAddon, InputGroupInput, InputGroupTextarea } from '../ui/input-group';
import { Label } from '../ui/label';
import { Button } from '../ui/button';
import { Switch } from '../ui/switch';
import { Slider } from '../ui/slider';
import { toast } from 'vue-sonner';

const isGeneratePassword = ref(false)
const generatePasswordOpt = reactive({
    length: [12],
    letters: true,
    digits: true,
    symbols: true,
})

const generatedPassword = ref('')
const copied = ref(false)

function generatePassword() {
    if (typeof window === 'undefined') return ''

    const lowercase = 'abcdefghijklmnopqrstuvwxyz'
    const uppercase = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
    const digits = '0123456789'
    const symbols = '@!&%*'

    let pool = lowercase
    const requiredChars: string[] = []

    if (generatePasswordOpt.letters) {
        pool += uppercase
        const randomIndex = Math.floor(Math.random() * uppercase.length)
        requiredChars.push(uppercase[randomIndex]!)
    }

    if (generatePasswordOpt.digits) {
        pool += digits
        const randomIndex = Math.floor(Math.random() * digits.length)
        requiredChars.push(digits[randomIndex]!)
    }

    if (generatePasswordOpt.symbols) {
        pool += symbols
        const randomIndex = Math.floor(Math.random() * symbols.length)
        requiredChars.push(symbols[randomIndex]!)
    }

    const randomIndex = Math.floor(Math.random() * lowercase.length)
    requiredChars.push(lowercase[randomIndex]!)

    const remainingLength = Math.max(0, generatePasswordOpt.length[0]! - requiredChars.length)

    const array = new Uint32Array(remainingLength)
    window.crypto.getRandomValues(array)

    const randomChars: string[] = []
    for (let i = 0; i < remainingLength; i++) {
        randomChars.push(pool.charAt(array[i]! % pool.length))
    }

    const allChars = [...requiredChars, ...randomChars]

    for (let i = allChars.length - 1; i > 0; i--) {
        const randomArray = new Uint32Array(1)
        window.crypto.getRandomValues(randomArray)
        const j = randomArray[0]! % (i + 1)
            ;[allChars[i], allChars[j]] = [allChars[j]!, allChars[i]!]
    }

    return allChars.join('')
}

function regeneratePassword() {
    generatedPassword.value = generatePassword()
    toast.success('Password regenerated!')
}

async function copyToClipboard() {
    try {
        await navigator.clipboard.writeText(generatedPassword.value)
        copied.value = true
        toast.success('Password copied to clipboard!')

        setTimeout(() => {
            copied.value = false
        }, 2000)
    } catch (err) {
        console.error('Failed to copy:', err)
        toast.error('Failed to copy password')
    }
}

watch(generatePasswordOpt, () => {
    generatedPassword.value = generatePassword()
}, { deep: true })

watch(isGeneratePassword, (isOpen) => {
    if (isOpen && !generatedPassword.value) {
        generatedPassword.value = generatePassword()
    }
})
</script>

<template>
    <div class="space-y-4">
        <div class="space-y-2">
            <Label>Login Details</Label>
            <InputGroup>
                <InputGroupAddon align="inline-start">
                    <Mail />
                </InputGroupAddon>
                <InputGroupInput placeholder="Username or email" />
            </InputGroup>
            <InputGroup>
                <InputGroupAddon align="inline-start">
                    <LockKeyhole />
                </InputGroupAddon>
                <InputGroupInput placeholder="Password" />
            </InputGroup>
        </div>

        <div>
            <Button v-if="!isGeneratePassword" variant="outline" class="w-full" type="button"
                @click="isGeneratePassword = !isGeneratePassword">
                <KeyRound />
                Generate Password
            </Button>
            <div v-else>
                <InputGroup>
                    <InputGroupAddon align="block-start">
                        <InputGroupButton variant="outline" class="flex-1 text-green-500 hover:text-green-600"
                            type="button" @click="isGeneratePassword = false">
                            <CircleCheck /> Use
                        </InputGroupButton>

                        <InputGroupButton variant="outline" class="flex-1" type="button" @click="regeneratePassword">
                            <RefreshCw class="w-4 h-4 mr-2" />
                            Regenerate
                        </InputGroupButton>

                        <InputGroupButton class="flex-1" type="button" @click="copyToClipboard" variant="outline">
                            <Check v-if="copied" class="w-4 h-4 mr-2" />
                            <Copy v-else class="w-4 h-4 mr-2" />
                            {{ copied ? 'Copied!' : 'Copy' }}
                        </InputGroupButton>
                        <InputGroupButton type="button" variant="destructive" size="icon-xs"
                            @click="isGeneratePassword = false">
                            <X />
                        </InputGroupButton>
                    </InputGroupAddon>
                    <InputGroupTextarea disabled :value="generatedPassword" class="text-white" />
                    <InputGroupAddon align="block-end" class="flex flex-col w-full cursor-default">
                        <div class="flex justify-between items-center w-full">
                            <Label for="length" class="flex">Length: {{ generatePasswordOpt.length[0] }}</Label>
                            <Slider :default-value="[12]" :min="8" :max="64" :step="1" class="w-[60%]"
                                v-model="generatePasswordOpt.length" />
                        </div>
                        <div class="flex justify-between items-center w-full">
                            <Label for="letters">Use capital letters (A-Z)</Label>
                            <Switch id="letters" v-model="generatePasswordOpt.letters" />
                        </div>
                        <div class="flex justify-between items-center w-full">
                            <Label for="digits">Use digits (0-9)</Label>
                            <Switch id="digits" v-model="generatePasswordOpt.digits" />
                        </div>
                        <div class="flex justify-between items-center w-full">
                            <Label for="symbols">Use symbols (@!&%*)</Label>
                            <Switch id="symbols" v-model="generatePasswordOpt.symbols" />
                        </div>
                    </InputGroupAddon>
                </InputGroup>

            </div>
        </div>

        <div class="space-y-2">
            <Label>Website or Apps</Label>
            <InputGroup>
                <InputGroupAddon align="inline-start">
                    <Globe />
                </InputGroupAddon>
                <InputGroupInput placeholder="Website or Apps" />
            </InputGroup>
        </div>
    </div>
</template>