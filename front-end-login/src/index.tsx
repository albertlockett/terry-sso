import React from 'react';
import ReactDOM from 'react-dom';

import './styles.scss';

function App() {
  return (
    <div className="login-page">
      <h1>Login</h1>
      <form method="post">
        <label>username</label>
        <input type="text" />
        <br />
        <label>password</label>
        <input type="text" />
        <br />
        <button>OK</button>
      </form>
    </div>
  );
}

ReactDOM.render(<App />, document.getElementById('app'));
