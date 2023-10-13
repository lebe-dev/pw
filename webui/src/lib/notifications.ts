import {toast} from '@zerodevx/svelte-toast';

export const showError = (message: string) => toast.push(message, {
    duration: 10000,
});