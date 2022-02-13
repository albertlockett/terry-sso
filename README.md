# terry-sso

This is the code for the "Click to Sign in with Terrylockett.ca".

It's an Implementation of the OATH2 PCKE flow for demonstration purposes only. Don't clone this and use it as production code.

There are 4 modules
 - `server` - the API server which implements the endponits
   - /oauth2/authorize
   - /oath2/login
   - /oath2/token
   - TODO document those
 - `front-end-login` - the front-end application of the login form
 - `front-end-sdk` - a Javascript SDK that can be used on 3rd party websites to handle the flow logic
 - `front-end-testsite` - a test site showing how to use the SDK
 - `back-end-testsite` - a test site showing how to implement an API that consumes the token
