import React from 'react';

type ErrorProps = {
  error: string;
};

const errorMessages = {
  invalid_credentials: 'Your password and/or username is wrong!!!!!!',
  invalid_session: 'Invalid session state!!!!!!!!!!!'
};

export default function Error(props: ErrorProps) {
  if (!props.error) {
    return null;
  }

  return (
    <div className="error">
      <b>Error happen:</b>
      <br />
      <em>{errorMessages[props.error]}</em>
    </div>
  );
}
