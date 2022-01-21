/**
 * do the login
 */
export async function doLogin() {
  const verifier = new Verifier();
  // TODO - do something with verifier

  window.location = 'http://localhost:4000/oauth2/authorize';
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
