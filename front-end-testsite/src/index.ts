import * as sdk from '../../front-end-sdk/dist/main';

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
