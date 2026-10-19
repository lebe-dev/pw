import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { Secret } from '$lib/secret';

export const load: PageLoad = ({ params }) => {
	if (params.id) {
		return {
			secretId: params.id,
			secret: new Secret()
		};
	}

	throw error(404, 'Not found');
};
