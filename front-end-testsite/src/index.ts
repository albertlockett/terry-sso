import * as sdk from '../../front-end-sdk/dist/main';

let token = '';

(async function () {
  if (!window.location.pathname.startsWith('/callback')) {
    return;
  }

  const params = new URLSearchParams(window.location.search);
  const code = params.get('code');
  const error = params.get('error');
  if (error) {
    handleError(error);
  } else {
    const result = await sdk.exchangeCode({ code });
    token = result.access_token;
  }
})();

async function handleError(error) {
  console.error(error);
  document.addEventListener('DOMContentLoaded', function () {
    const errorContainer = document.getElementById('error');
    errorContainer.innerText = 'An error occurred';
  });
}

const loginButton = document.getElementById('login');
loginButton.addEventListener(
  'click',
  function () {
    const scopes = ['openid', 'email', 'read_data'];
    sdk.doLogin({
      callbackUrl: `${window.location.origin}/callback`,
      scopes,
      audience: 'https://some-service.example.com'
    });
  },
  false
);

const button = document.getElementById('button');
button.addEventListener(
  'click',
  async function () {
    const res = await fetch('http://localhost:4001/v1/data', {
      mode: 'cors',
      headers: {
        Authorization: `Bearer ${token}`
      }
    });
    const text = await res.text();
    console.log(text);
  },
  false
);

console.log('here');
