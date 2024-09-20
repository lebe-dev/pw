import {notifications} from "$lib/stores/notifications";

export const showError = (message: string) => {
    notifications.addError({message: message});
};