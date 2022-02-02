import React from 'react';
import ReactDOM from 'react-dom';
import Error from './Error';

import './styles.scss';

function App() {
  const params = new URLSearchParams(window.location.search);
  const sessionId = params.get('session_id');
  const error = params.get('error');
  return (
    <div className="login-page">
      <h1>
        <em>terrylockett.ca LOGON</em>
      </h1>
      <form
        className="form"
        method="post"
        action="http://localhost:4000/oauth2/login"
      >
        <label>
          <blink>username: </blink>
        </label>
        <input type="text" name="username" />
        <br />
        <br />
        <label>password: </label>
        <input type="password" name="password" />
        <br />
        <br />
        <input type="hidden" name="session_id" value={sessionId} />
        <button>OK</button>
        <br />
        <Error error={error} />
      </form>

      <div className="pics">
        <img
          src="http://terrylockettca.s3-website.us-east-2.amazonaws.com/images/snoopGif1.gif"
          width="200"
          height="200"
        />
        <img
          src="http://terrylockettca.s3-website.us-east-2.amazonaws.com/images/pebs.jpg"
          width="200"
          height="200"
        />
        <img
          src="http://terrylockettca.s3-website.us-east-2.amazonaws.com/images/catsMcDonalds.gif"
          width="200"
          height="200"
        />
      </div>
    </div>
  );
}

ReactDOM.render(<App />, document.getElementById('app'));
