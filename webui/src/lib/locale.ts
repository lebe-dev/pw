export class Locale {
    id: string = 'en';

    messages: MessageLabels = new MessageLabels();

    errors: ErrorLabels = new ErrorLabels();

    homePage: HomePageLabels = new HomePageLabels();

    secretUrlPage: SecretUrlPageLabels = new SecretUrlPageLabels();

    secretNotFoundPage: SecretNotFoundPageLabels = new SecretNotFoundPageLabels();

    footerLabels: FooterLabels = new FooterLabels();
}

export class MessageLabels {
    loadingTitle: string = 'Loading..';
    errorTitle: string = 'Error';
}

export class ErrorLabels {
    loadingData: string = 'Couldn\'t load data';
    storeSecret: string = 'Store secret error';
}

export class HomePageLabels {
    title: string = 'Message';
    messagePlaceholder: string = 'The data will be encrypted in the browser';
    secretLifetimeTitle: string = 'Secret lifetime';
    lifetime: LifetimeLabels = new LifetimeLabels();
    encryptMessageButton: string = 'Encrypt message';
    secretUrlTitle: string = 'Secret URL'
    copyButton: string = 'Copy';
}

export class LifetimeLabels {
    oneHour: string = 'One hour';
    twoHours: string = 'Two hours';
    oneDay: string = 'One day';
    oneWeek: string = 'One week';
    oneTimeDownload: string = 'One time download';
    oneTimeDownloadPrecautionMessage: string = 'This link is for one-time use only, so don\'t try to open it or the secret will disappear.';
}

export class SecretUrlPageLabels {
    title: string = 'Message';
    oneTimeDownloadPrecautionMessage: string = 'This link is for one-time use only, so don\'t try to open it or the secret will disappear.';
}

export class SecretNotFoundPageLabels {
    title: string = 'Secret wasn\'t found';

    possibleReasonsText: string = 'Possible reasons';

    possibleReasonsItems: string[] = ['Link has been expired', 'It was one-time link and someone opened it already'];
}

export class FooterLabels {
    howItWorks: string = 'FAQ';
}