export class Secret {
	id: string = '';
	payload: string = '';
	contentType: SecretContentType = SecretContentType.Text;
	ttl: SecretTTL = SecretTTL.OneHour;
	downloadPolicy: SecretDownloadPolicy = SecretDownloadPolicy.OneTime;
	metadata?: FileMetadata;
}

export enum SecretContentType {
	Text = 'Text',
	File = 'File'
}

export enum SecretTTL {
	OneHour = 'OneHour',
	TwoHours = 'TwoHours',
	OneDay = 'OneDay',
	OneWeek = 'OneWeek'
}

export enum SecretDownloadPolicy {
	OneTime = 'OneTime',
	Unlimited = 'Unlimited'
}

export class FileMetadata {
	name: string = '';
	type: string = '';
	size: number = 0;
}
