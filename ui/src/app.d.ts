declare module 'qrious' {
	export default class QRious {
		constructor(options: { element: HTMLCanvasElement; value: string; size: number });
	}
}

declare namespace App {
	interface Error {
		message: string;
		detail?: string;
	}

	// interface Locals {}

	// interface PageData {}

	// interface Platform {}
}
