import * as sdk from '../../front-end-sdk/dist/main';

let token = '';

(async function () {
  if (!window.location.pathname.startsWith('/callback')) {
    return;
  }

  const params = new URLSearchParams(window.location.search);
  const code = params.get('code');
  // TODO error checking on code

  const result = await sdk.exchangeCode({ code });
  console.log({ result });
  token = result.access_token;
})();

const button = document.getElementById('login');
button.addEventListener(
  'click',
  function () {
    sdk.doLogin({
      callbackUrl: `${window.location.origin}/callback`
    });
  },
  false
);
