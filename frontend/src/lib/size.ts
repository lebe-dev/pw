export function getPrettySize(
	value: string,
	kbLabel: string,
	mbLabel: string,
	gbLabel: string
): string {
	let result;

	const intValue = parseInt(value, 0.0);

	if (intValue <= 1000) {
		result = value + ' Б';
	} else if (intValue < 1000000 && parseInt(value, 0.0) > 999) {
		result = (intValue / 1000).toFixed(1) + ' ' + kbLabel;
	} else if (intValue < 1000000000 && parseInt(value, 0.0) > 999999) {
		result = (intValue / 1000 / 1000).toFixed(1) + ' ' + mbLabel;
	} else if (intValue >= 1000000000) {
		result = (intValue / 1000 / 1000 / 1000).toFixed(1) + ' ' + gbLabel;
	} else {
		result = intValue.toFixed(1) + ' ' + kbLabel;
	}

	return result;
}
