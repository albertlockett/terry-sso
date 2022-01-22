type LoginParams = {
  callbackUrl: string;
};

/**
 * do the login
 */
export async function doLogin(params: LoginParams) {
  const verifier = new Verifier();
  document.cookie = `terry_auth=${verifier}; new Date(new Date().getTime() + 300_000); path=/`;

  const challenge = await verifier.getChallenge();
  window.location = `http://localhost:4000/oauth2/authorize?challenge=${challenge}&callback_url=${params.callbackUrl}`;
}

type ExchangeCodeParams = {
  code: string;
};

type ExchangeCodeResponse = {
  access_token: string;
  id_token: string;
};

export async function exchangeCode(
  params: ExchangeCodeParams
): ExchangeCodeResponse {
  console.log({ params });
  const verifier = document.cookie
    .split(';')
    .map((s) => s.trim())
    .find((c) => c.startsWith('terry_auth'))
    .replace('terry_auth=', '');
  const response = await fetch('http://localhost:4000/oauth2/token', {
    method: 'POST'
  });
  console.log(await response.text());
}

/**
 * our implementation of the code verifier
 */
export class Verifier {
  constructor() {
    this.array = new Uint8Array(64);
    window.crypto.getRandomValues(this.array);
    this.getChallenge = this.getChallenge.bind(this);
  }

  async getChallenge(): string {
    return toBase64(await sha256(this.array));
  }

  toString() {
    return toBase64(this.array);
  }
}

// helper functions below ...

async function sha256(bytes: Uint8Array): Uint8Array {
  const hash = await crypto.subtle.digest('SHA-256', bytes);
  return new Uint8Array(hash);
}

function toBase64(bytes: Uint8Array): string {
  let binary = '';
  for (let i = 0; i < bytes.byteLength; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}
