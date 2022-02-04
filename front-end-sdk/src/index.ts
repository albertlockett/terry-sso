type LoginParams = {
  callbackUrl: string;
  audience?: string;
  scopes?: string[];
};

export async function doLogin(params: LoginParams) {
  // store the verifier in the cookie for later use.
  // It expires in 5 minutes, hopefully it doesn't take that long for users to type their password etc.
  const verifier = new Verifier();
  document.cookie = `terry_auth=${verifier}; new Date(new Date().getTime() + 300_000); path=/`;

  const challenge = await verifier.getChallenge();
  let queryString = `challenge=${challenge}&callback_url=${params.callbackUrl}`;
  queryString += `&audience=${params.audience ?? ''}`;
  queryString += `&scopes=${params.scopes?.join(',') ?? ''}`;
  window.location = `http://localhost:4000/oauth2/authorize?${queryString}`;
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
  const verifier = getVerifierFromCookie(document.cookie);
  const response = await fetch('http://localhost:4000/oauth2/token', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ code: params.code, verifier })
  });
  const result = await response.json();
  return result;
}

function getVerifierFromCookie(cookie): string {
  const verifier = cookie
    .split(';')
    .map((s) => s.trim())
    .find((c) => c.startsWith('terry_auth'))
    .replace('terry_auth=', '');
  return verifier;
}

/**
 * our implementation of the code verifier
 */
export class Verifier {
  constructor() {
    this.array = new Uint8Array(32);
    window.crypto.getRandomValues(this.array);
    this.getChallenge = this.getChallenge.bind(this);
  }

  async getChallenge(): string {
    const sha = await sha256(this.array);
    console.log('sha', sha);
    return toBase64(sha);
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
  return window.btoa(binary).replaceAll(' ', '+');
}
