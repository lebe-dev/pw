import {writable} from 'svelte/store';

export type Notification = {
	id?: string;
	timeout?: number;
	message: string;
	type?: 'error' | 'warn' | 'info';
};

function createNotificationStore() {
	const { subscribe, update } = writable<Array<Notification>>([]);

	const remove = (id: string) => {
		update((notifications) => notifications.filter((n) => n.id !== id));
	};

	return {
		subscribe,
		add: (notification: Omit<Notification, 'id'>) => {
			const id = Math.random().toString(36).substr(2, 9);
			update((notifications) => [...notifications, { id, ...notification }]);
			if (notification.timeout) {
				setTimeout(() => {
					remove(id);
				}, notification.timeout);
			}
		},
		addInfo: (notification: Omit<Notification, 'id'>) => {
			const id = Math.random().toString(36).substr(2, 9);
			const type = 'info';
			update((notifications) => [...notifications, { id, type, ...notification }]);
			if (notification.timeout) {
				setTimeout(() => {
					remove(id);
				}, notification.timeout);
			}
		},
		addWarning: (notification: Omit<Notification, 'id'>) => {
			const id = Math.random().toString(36).substr(2, 9);
			const type = 'warn';
			update((notifications) => [...notifications, { id, type, ...notification }]);
			if (notification.timeout) {
				setTimeout(() => {
					remove(id);
				}, notification.timeout);
			}
		},
		addError: (notification: Omit<Notification, 'id'>) => {
			const id = Math.random().toString(36).substr(2, 9);
			const type = 'error';
			update((notifications) => [...notifications, { id, type, ...notification }]);
			if (notification.timeout) {
				setTimeout(() => {
					remove(id);
				}, notification.timeout);
			}
		},
		remove: (id: string) => {
			update((notifications) => notifications.filter((n) => n.id !== id));
		}
	};
}

export const notifications = createNotificationStore();