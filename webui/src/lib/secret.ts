export class Secret {
    id: string = '';
    payload: string = '';
    ttl: SecretTTL = SecretTTL.OneHour;
    downloadPolicy: SecretDownloadPolicy = SecretDownloadPolicy.OneTime;

}

export enum SecretTTL {
    OneHour = 'OneHour',
    TwoHours = 'TwoHours',
    OneDay = 'OneDay'
}

export enum SecretDownloadPolicy {
    OneTime = 'OneTime',
    Unlimited = 'Unlimited'
}