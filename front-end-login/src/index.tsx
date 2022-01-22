import React from 'react';
import ReactDOM from 'react-dom';

import './styles.scss';

function App() {
  const params = new URLSearchParams(window.location.search);
  const challenge = params.get('challenge');
  const callbackUrl = params.get('callbackUrl');
  return (
    <div className="login-page">
      <h1>Login</h1>
      <form method="post" action="http://localhost:4000/oauth2/login">
        <label>username</label>
        <input type="text" name="username" />
        <br />
        <label>password</label>
        <input type="password" name="password" />
        <br />
        <input type="hidden" name="challenge" value={challenge} />
        <input type="hidden" name="callback_url" value={callbackUrl} />
        <button>OK</button>
      </form>
    </div>
  );
}

ReactDOM.render(<App />, document.getElementById('app'));
