use poem::handler;
use poem::web::Json;

use crate::http::middlewares::auth::methods::create_jwt;
use crate::http::middlewares::auth::models::{Claims, UserPermissions};

// An additional handler for generating a token.
// Thus, you can try to create your own tokens and check the operation of the permissions' system.
// cURL example:
// ```sh
//  curl --location --request POST 'http://localhost:3000/token' \
//   --header 'Content-Type: application/json' \
//   --data-raw '{
//       "username": "Lorem-Ipsum",
//       "permissions": ["OP_GET_SECURED_INFO"]
//   }'
// ```
#[handler]
pub async fn create_token(info: Json<UserPermissions>) -> poem::Result<String> {
    let user_info = info.0;
    // Create a JWT
    let claims = Claims::new(user_info.username, user_info.permissions);
    let jwt = create_jwt(claims)?;

    // Return token for work with example handlers
    Ok(jwt)
}